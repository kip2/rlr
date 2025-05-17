use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::Arc,
};

use directories::ProjectDirs;
use regex::Regex;
use reqwest::{
    Url,
    blocking::{Client, Response},
    cookie::{CookieStore, Jar},
    header::LOCATION,
};
use scraper::{Html, Selector};

use crate::{
    error::Error,
    file::save_to_file,
    messages::{FAILED_LABEL, INFO_LABEL, NETWORK_LABEL, SUCCESS_LABEL},
    parser::{get_test_cases, save_test_cases},
};

type Cookie = HashMap<String, String>;
type HTML = String;

enum Redirect {
    ON,
    OFF,
}

pub fn initial_auth(email: &str, password: &str) -> Result<(), Error> {
    println!("[{}] Start Login process.", *INFO_LABEL);

    let jar = Arc::new(Jar::default());
    let client_get = create_client(Redirect::ON, &jar)?;

    let request_path = "https://recursionist.io/login";
    println!("[{}] GET: {}", *NETWORK_LABEL, request_path);

    let res = client_get.get(request_path).send()?;
    println!("[{}] {}", *NETWORK_LABEL, res.status());

    let login_html = res.text()?;

    let token = extract_token_from_html(&login_html)?;

    let client_post = create_client(Redirect::OFF, &jar)?;

    let mut form = HashMap::new();
    form.insert("email", email);
    form.insert("password", password);
    form.insert("_token", &token);

    println!("[{}] Send Login request...", *INFO_LABEL);
    println!("[{}] POST: {}", *NETWORK_LABEL, request_path);

    let res = client_post
        .post(request_path)
        .form(&form)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", "https://recursionist.io/")
        .header("Origin", "https://recursionist.io")
        .send()?;

    println!("[{}] {}", *NETWORK_LABEL, res.status());

    let location = res
        .headers()
        .get(LOCATION)
        .ok_or(Error::HeaderMissing(
            "Location header missing in initial_auth".to_string(),
        ))?
        .to_str()
        .map_err(|_| {
            Error::Internal("Failed to parse Location header as UTF-8 in initial_auth".to_string())
        })?;

    if is_login_successful(location) {
        println!("[{}] Login sucess.", *SUCCESS_LABEL);
        let url = Url::parse("https://recursionist.io").map_err(|_| Error::UrlIncorrectFormat)?;
        let cookies = jar.cookies(&url).ok_or(Error::CookieMissing)?;
        let cookie_str = cookies
            .to_str()
            .map_err(|_| Error::CookieNotUtf8)?
            .to_string();

        save_cookie_to_file(cookie_str)?;
        Ok(())
    } else {
        println!("[{}] Login failed.", *FAILED_LABEL);
        Err(Error::LoginFailed)
    }
}

pub fn download(arg_s: &str) -> Result<(), Error> {
    let url = if is_natural_number(arg_s) {
        create_url(arg_s)
    } else {
        arg_s.to_string()
    };

    if !valid_problem_url(&url)? {
        return Err(Error::UrlIncorrectFormat);
    }

    let html = fetch_problem_page(&url)?;

    let problem_id = extract_url_number(&url)?;

    let test_cases = get_test_cases(&html)?;
    save_test_cases(test_cases, &problem_id)?;

    Ok(())
}

fn valid_problem_url(url: &str) -> Result<bool, Error> {
    let re = Regex::new(r"^https://recursionist.io/dashboard/problems/\d+$")
        .map_err(|_| Error::Internal("Regex compile error in valid_email".to_string()))?;

    Ok(re.is_match(url.trim()))
}

fn is_login_successful(location: &str) -> bool {
    location == "https://recursionist.io/dashboard"
}

fn get_cookie_path() -> Result<std::path::PathBuf, Error> {
    if let Some(project_dir) = ProjectDirs::from("Recursion", "tool", "rlr") {
        let config_dir = project_dir.config_dir();
        Ok(config_dir.join("cookie.jar"))
    } else {
        Err(Error::CookiePathUnvaliable)
    }
}

fn create_url(num_str: &str) -> String {
    let prefix = "https://recursionist.io/dashboard/problems/";
    format!("{}{}", prefix, num_str)
}

fn is_natural_number(s: &str) -> bool {
    s.parse::<u32>().is_ok()
}

fn extract_url_number(url: &str) -> Result<String, Error> {
    let re = Regex::new(r"/problems/(\d+)$")
        .map_err(|_| Error::Internal("Regex compile error in extract_url_number".to_string()))?;
    let caps = re.captures(url).ok_or(Error::UrlIncorrectFormat)?;
    let matched = caps.get(1).ok_or(Error::Internal(format!(
        "Capture group not found in extract_url_number for URL: {}",
        url
    )))?;
    Ok(matched.as_str().to_string())
}

fn fetch_problem_page(url: &str) -> Result<HTML, Error> {
    let jar = Arc::new(Jar::default());
    let client = create_client(Redirect::ON, &jar)?;

    let cookie_path = get_cookie_path()?;

    let cookies = load_cookies(cookie_path)?;
    let cookie_header = format_cookie_header(cookies);

    println!("[{}] GET: {}", *NETWORK_LABEL, url);
    let res = get_page_with_cookie(&client, url, &cookie_header)?;

    let final_url = res.url().as_str();
    if final_url != url {
        return Err(Error::Internal(
            "Unexpected redirect in fetch_problem_page".to_string(),
        ));
    }

    println!("[{}] {}", *NETWORK_LABEL, res.status());
    println!();

    let url_parsed = Url::parse(url).map_err(|_| Error::UrlIncorrectFormat)?;

    let cookie = jar.cookies(&url_parsed).ok_or(Error::NoCookie)?;

    let response_cookie = cookie
        .to_str()
        .map_err(|_| Error::CookieNotUtf8)?
        .to_string();

    save_cookie_to_file(response_cookie)?;

    let body = res.text()?;
    Ok(body)
}

fn save_cookie_to_file(cookie: String) -> Result<(), Error> {
    let cookie_path = get_cookie_path()?;
    save_to_file(&cookie_path, &cookie)?;
    println!("[{}] Save cookie to: {:?}", *INFO_LABEL, cookie_path);
    Ok(())
}

fn create_client(disable_redirect: Redirect, jar: &Arc<Jar>) -> Result<Client, Error> {
    let builder = Client::builder().cookie_provider(jar.clone());

    Ok(match disable_redirect {
        Redirect::ON => builder,
        Redirect::OFF => builder.redirect(reqwest::redirect::Policy::none()),
    }
    .build()?)
}

fn format_cookie_header(cookies: Cookie) -> String {
    let key_order = ["recursion_session", "XSRF-TOKEN"];
    key_order
        .iter()
        .filter_map(|k| cookies.get(*k).map(|v| format!("{}={}", k, v)))
        .collect::<Vec<_>>()
        .join("; ")
}

fn get_page_with_cookie(
    client: &Client,
    url: &str,
    cookie_header: &str,
) -> Result<Response, Error> {
    let res = client.get(url).header("Cookie", cookie_header).send()?;
    Ok(res)
}

fn extract_token_from_html(html: &str) -> Result<String, Error> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse(r#"input[name="_token"]"#).map_err(|_| {
        Error::TokenNotFound("Token parse error in extract_token_from_html".to_string())
    })?;
    let token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value").map(|v| v.to_string()))
        .map(|v| v.to_string())
        .ok_or(Error::TokenNotFound(
            "Token parse error in extract_token_from_html".to_string(),
        ))?;

    Ok(token)
}

fn load_cookies<P: AsRef<Path>>(path: P) -> Result<Cookie, Error> {
    println!("[{}] Load cookie from: {:?}", *INFO_LABEL, path.as_ref());
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut cookies = Cookie::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once("=") {
            cookies.insert(key.trim().to_string(), value.trim().to_string());
        } else {
            return Err(Error::MalformedCookie(line));
        }
    }

    Ok(cookies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_url() {
        let num_str = "1";
        let actual = create_url(num_str);

        let expected = "https://recursionist.io/dashboard/problems/1";

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_is_natural_number() {
        let num_str = "1";

        assert!(is_natural_number(num_str));

        let num_str = "-1";

        assert!(!is_natural_number(num_str));

        let num_str = "0";

        assert!(is_natural_number(num_str));

        let num_str = "1.5";

        assert!(!is_natural_number(num_str));
    }

    #[test]
    fn test_extract_url_number() {
        let url = "https://example.com/dashboard/pjroblems/42";
        assert_eq!(extract_url_number(url).unwrap(), "42".to_string());
    }

    #[test]
    fn test_load_cookies() {
        let path = "./tests/cookie.jar";

        let actual = load_cookies(path).unwrap();

        let mut expected: HashMap<String, String> = HashMap::new();

        expected.insert(
            "recursion_session".to_string(),
            "recursion_session".to_string(),
        );
        expected.insert("XSRF-TOKEN".to_string(), "xsrf-token".to_string());

        assert_eq!(
            actual.get("recursion_session"),
            expected.get("recursion_session")
        );

        assert_eq!(actual.get("XSRF-TOKEN"), expected.get("XSRF-TOKEN"));
    }

    #[test]
    fn test_format_cookie_header() {
        let path = "./tests/cookie.jar";

        let cookies = load_cookies(path).unwrap();

        let actual = format_cookie_header(cookies);

        let expected = "recursion_session=recursion_session; XSRF-TOKEN=xsrf-token";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_login_successful() {
        let location = "https://recursionist.io/dashboard";

        assert!(is_login_successful(location));

        let wrong_location = "https://recursionist.io/";

        assert!(!is_login_successful(wrong_location));
    }

    #[test]
    fn test_valid_problem_url() {
        let url = "https://recursionist.io/dashboard/problems/1";

        assert!(valid_problem_url(url).unwrap());

        let url = "https://recursionist.io/dashboard/problems/1000";

        assert!(valid_problem_url(url).unwrap());

        let url = "https://example.com/dashboard/problems/1";

        assert!(!valid_problem_url(url).unwrap());

        let url = "123https://recursionist.io/dashboard/problems/1";

        assert!(!valid_problem_url(url).unwrap());

        let url = "https://recursionist.io/dashboard/problems/100.000";

        assert!(!valid_problem_url(url).unwrap());
    }
}

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
    error::{Error, handle_error},
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

pub fn get_cookie_path() -> Option<std::path::PathBuf> {
    if let Some(project_dir) = ProjectDirs::from("Recursion", "tool", "rlr") {
        let config_dir = project_dir.config_dir();
        Some(config_dir.join("cookie.jar"))
    } else {
        None
    }
}

pub fn initial_auth(email: &str, password: &str) {
    println!("[{}] Start Login process.", *INFO_LABEL);

    let jar = Arc::new(Jar::default());
    let client_get = create_client(Redirect::ON, &jar);

    let request_path = "https://recursionist.io/login";
    println!("[{}] GET: {}", *NETWORK_LABEL, request_path);

    let res = client_get.get(request_path).send().unwrap();
    println!("[{}] {}", *NETWORK_LABEL, res.status());

    let login_html = res.text().unwrap();

    let token = extract_token_from_html(&login_html).expect("CSRF _token not found");

    let client_post = create_client(Redirect::OFF, &jar);

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
        .send()
        .unwrap();

    println!("[{}] {}", *NETWORK_LABEL, res.status());

    let location = res
        .headers()
        .get(LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if is_login_successful(location) {
        println!("[{}] Login sucess.", *SUCCESS_LABEL);
        let url = Url::parse("https://recursionist.io").unwrap();
        let cookies = jar.cookies(&url).unwrap();
        let cookie_str = cookies.to_str().unwrap().to_string();

        save_cookie_to_file(cookie_str).unwrap();
    } else {
        println!("[{}] Login failed.", *FAILED_LABEL);
    }
}

fn is_login_successful(location: &str) -> bool {
    location == "https://recursionist.io/dashboard"
}

pub fn download(arg_s: &str) {
    let url = if is_natural_number(arg_s) {
        create_url(arg_s)
    } else {
        arg_s.to_string()
    };

    let html = fetch_problem_page(&url).unwrap();

    let problem_id = extract_url_number(&url);

    match get_test_cases(&html) {
        Ok(test_cases) => {
            if let Err(e) = save_test_cases(test_cases, &problem_id) {
                handle_error(e);
            }
        }
        Err(e) => handle_error(e),
    }
}

fn create_url(num_str: &str) -> String {
    let prefix = "https://recursionist.io/dashboard/problems/";
    format!("{}{}", prefix, num_str)
}

fn is_natural_number(s: &str) -> bool {
    s.parse::<u32>().is_ok()
}

fn extract_url_number(url: &str) -> String {
    let re = Regex::new(r"/problems/(\d+)$").unwrap();
    let caps = re.captures(url).unwrap();
    let matched = caps.get(1).unwrap();
    matched.as_str().to_string()
}

fn fetch_problem_page(url: &str) -> Result<HTML, Error> {
    let jar = Arc::new(Jar::default());
    let client = create_client(Redirect::ON, &jar);

    let cookie_path = get_cookie_path().unwrap();

    let cookies = load_cookies(cookie_path).unwrap();
    let cookie_header = format_cookie_header(cookies);

    println!("[{}] GET: {}", *NETWORK_LABEL, url);
    let res = get_page_with_cookie(&client, url, &cookie_header).unwrap();

    let final_url = res.url().as_str();
    if final_url != url {
        return Err(Error::Unexpectedredirect {
            expected: url.to_string(),
            actual: final_url.to_string(),
        });
    }

    println!("[{}] {}", *NETWORK_LABEL, res.status());
    println!();

    let response_cookie = jar
        .cookies(&Url::parse(url).unwrap())
        .ok_or("No cookies")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    save_cookie_to_file(response_cookie).unwrap();

    let body = res.text().unwrap();
    Ok(body)
}

fn save_cookie_to_file(cookie: String) -> Result<(), Error> {
    let cookie_path = get_cookie_path().ok_or(Error::CookiePathMissing)?;
    save_to_file(&cookie_path, &cookie)?;
    println!("[{}] Save cookie to: {:?}", *INFO_LABEL, cookie_path);
    Ok(())
}

fn create_client(disable_redirect: Redirect, jar: &Arc<Jar>) -> Client {
    let builder = Client::builder().cookie_provider(jar.clone());

    match disable_redirect {
        Redirect::ON => builder,
        Redirect::OFF => builder.redirect(reqwest::redirect::Policy::none()),
    }
    .build()
    .expect("failed to create client")
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
) -> Result<Response, reqwest::Error> {
    client.get(url).header("Cookie", cookie_header).send()
}

fn extract_token_from_html(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse(r#"input[name="_token"]"#).ok()?;
    doc.select(&selector)
        .next()
        .and_then(|e| e.value().attr("value").map(|v| v.to_string()))
}

fn load_cookies<P: AsRef<Path>>(path: P) -> Result<Cookie, Box<dyn std::error::Error>> {
    println!("[{}] Load cookie from: {:?}", *INFO_LABEL, path.as_ref());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut cookies = Cookie::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((key, value)) = line.split_once("=") {
            cookies.insert(key.trim().to_string(), value.trim().to_string());
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
        let url = "https://example.com/dashboard/problems/42";
        assert_eq!(extract_url_number(url), "42".to_string());
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
}

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::Arc,
};

use reqwest::{
    Url,
    blocking::{Client, Response},
    cookie::{CookieStore, Jar},
};
use scraper::{Html, Selector};

use crate::{env::read_env, error::Error, file::save_to_file};

type Cookie = HashMap<String, String>;
type HTML = String;

// todo: あとで設定ファイル読み込みでパスを繋げられるようにしておく
const COOKIE_PATH: &str = "./cookie/cookiejar";

pub fn initial_auth() {
    let jar = Arc::new(Jar::default());
    let client = create_client(&jar);

    let login_html = client
        .get("https://recursionist.io/login")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let token = extract_token_from_html(&login_html).expect("CSRF _token not found");

    let email = read_env("EMAIL");
    let password = read_env("PASSWORD");

    let mut form = HashMap::new();
    form.insert("email", email);
    form.insert("password", password);
    form.insert("_token", token);

    client
        .post("https://recursionist.io/login")
        .form(&form)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", "https://recursionist.io/")
        .header("Origin", "https://recursionist.io")
        .send()
        .unwrap();

    let url = Url::parse("https://recursionist.io").unwrap();
    let cookies = jar.cookies(&url).unwrap();
    let cookie_str = cookies.to_str().unwrap().to_string();

    save_cookie_to_file(cookie_str);
}

pub fn fetch_problem_page(url: &str) -> HTML {
    let jar = Arc::new(Jar::default());
    let client = create_client(&jar);

    let cookies = load_cookies(COOKIE_PATH).unwrap();
    let cookie_header = format_cookie_header(cookies);

    let res = get_page_with_cookie(&client, url, &cookie_header).unwrap();

    let response_cookie = jar
        .cookies(&Url::parse(url).unwrap())
        .ok_or("No cookies")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    save_cookie_to_file(response_cookie);

    let body = res.text().unwrap();
    body
}

fn save_cookie_to_file(cookie: String) -> Result<(), Error> {
    save_to_file(COOKIE_PATH, &cookie)?;
    Ok(())
}

fn create_client(jar: &Arc<Jar>) -> Client {
    Client::builder()
        .cookie_provider(jar.clone())
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

fn load_cookies(path: &str) -> Result<Cookie, Box<dyn std::error::Error>> {
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
    fn test_load_cookies() {
        let path = "./tests/cookiejar";

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
        let path = "./tests/cookiejar";

        let cookies = load_cookies(path).unwrap();

        let actual = format_cookie_header(cookies);

        let expected = "recursion_session=recursion_session; XSRF-TOKEN=xsrf-token";

        assert_eq!(actual, expected);
    }
}

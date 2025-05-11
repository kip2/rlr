use crate::error::Error;
use log::Log;
use regex::Regex;
use scraper::{Html, Selector};

use crate::file::save_to_file;

#[derive(Debug, PartialEq)]
pub struct TestCase {
    input: Vec<String>,
    output: String,
}

impl TestCase {
    fn new(input: Vec<String>, output: String) -> Self {
        Self { input, output }
    }
}

pub fn save_test_cases(test_cases: Vec<TestCase>, problem_id: &str) -> Result<(), Error> {
    let prefix_path = format!("./problem-{}/testcase/", problem_id);
    let prefix_file = "testcase-";
    let suffix_input_file = ".in";
    let suffix_output_file = ".out";

    for (i, case) in test_cases.iter().enumerate() {
        let index = i + 1;
        // save input file
        let input_file_path = format!(
            "{}{}{}{}",
            prefix_path, prefix_file, index, suffix_input_file
        );
        let input_flle_content = format_vec_str(&case.input);
        save_to_file(&input_file_path, &input_flle_content)?;

        // save output file
        let output_file_path = format!(
            "{}{}{}{}",
            prefix_path, prefix_file, index, suffix_output_file
        );
        let output_file_contnet = &case.output;
        save_to_file(&output_file_path, &output_file_contnet)?;
    }

    Ok(())
}

fn format_vec_str(s: &Vec<String>) -> String {
    s.iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn get_test_cases(html: &str) -> Result<Vec<TestCase>, Error> {
    let raw_test_cases = pickup_test_case(html)?;

    let result: Result<Vec<TestCase>, Error> = raw_test_cases
        .into_iter()
        .map(|line| {
            let inputs = parse_input_case(&line)?
                .split(",")
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            let output = parse_output_case(&line)?;
            Ok(TestCase::new(inputs, output))
        })
        .collect();

    result
}

fn selector_error<E: std::fmt::Display>(e: E) -> Error {
    Error::Selector(e.to_string().into())
}

fn pickup_test_case(html: &str) -> Result<Vec<String>, Error> {
    let document = Html::parse_document(html);

    let p_selector = Selector::parse("p").map_err(selector_error)?;
    let span_selector = Selector::parse("span[data-case]").map_err(selector_error)?;

    let mut targets = Vec::<String>::new();

    for p_elem in document.select(&p_selector) {
        let has_target_span = p_elem
            .select(&span_selector)
            .any(|span| span.value().attr("data-case").is_some());

        if has_target_span {
            let text = p_elem
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            targets.push(text);
        }
    }
    Ok(targets)
}

fn get_input_cases(test_cases: Vec<String>) -> Result<Vec<String>, Error> {
    test_cases
        .iter()
        .map(|case| parse_input_case(case))
        .collect()
}

fn parse_input_case(test_case: &str) -> Result<String, Error> {
    let re = match Regex::new(r"\(([^)]+)\)") {
        Ok(re) => re,
        Err(e) => {
            log::error!("正規表現の構文エラー: {}", e);
            return Err(Error::Internal);
        }
    };

    let cap = re.captures(&test_case).ok_or(Error::RegexCapture)?;
    let inside = &cap[1];
    let result = inside
        .split(",")
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join(",")
        .to_string();
    Ok(result)
}

fn get_output_cases(test_cases: Vec<String>) -> Result<Vec<String>, Error> {
    test_cases
        .iter()
        .map(|case| parse_output_case(case))
        .collect()
}

fn parse_output_case(test_case: &str) -> Result<String, Error> {
    let re = Regex::new(r"--> (.+)$")?;

    let cap = re.captures(&test_case).ok_or(Error::RegexCapture)?;
    let result = cap[1].to_string();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_vec_str() {
        let v = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let actual = format_vec_str(&v);
        let expected = "1 2 3".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_test_cases() {
        let html = r#"
        <p class="m-0 rem0p8">
            getLowestTemperature(3,2) --> 1
            <span class="cursor-pointer judge-test-button ml-1" data-case="[3,2]">
                <i class="far fa-play-circle" title="テスト実行"></i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature(2, 10) --> -8
            <span class="cursor-pointer judge-test-button ml-1" data-case="[2,10]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature( 18,5 ) --> 13
            <span class="cursor-pointer judge-test-button ml-1" data-case="[18,5]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        "#;

        let results = get_test_cases(html).unwrap();

        let expecteds = vec![
            TestCase::new(vec!["3".to_string(), "2".to_string()], "1".to_string()),
            TestCase::new(vec!["2".to_string(), "10".to_string()], "-8".to_string()),
            TestCase::new(vec!["18".to_string(), "5".to_string()], "13".to_string()),
        ];

        assert_eq!(results, expecteds);
    }

    #[test]
    fn test_get_output_test_cases() {
        let html = r#"
        <p class="m-0 rem0p8">
            getLowestTemperature(3,2) --> 1
            <span class="cursor-pointer judge-test-button ml-1" data-case="[3,2]">
                <i class="far fa-play-circle" title="テスト実行"></i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature(2, 10) --> -8
            <span class="cursor-pointer judge-test-button ml-1" data-case="[2,10]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature( 18,5 ) --> 13
            <span class="cursor-pointer judge-test-button ml-1" data-case="[18,5]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        "#;

        let results = pickup_test_case(html).unwrap();
        let input_cases = get_output_cases(results).unwrap();

        let expecteds = ["1", "-8", "13"];

        for (i, actual) in input_cases.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }

    #[test]
    fn test_get_input_test_cases() {
        let html = r#"
        <p class="m-0 rem0p8">
            getLowestTemperature(3,2) --> 1
            <span class="cursor-pointer judge-test-button ml-1" data-case="[3,2]">
                <i class="far fa-play-circle" title="テスト実行"></i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature(2, 10) --> -8
            <span class="cursor-pointer judge-test-button ml-1" data-case="[2,10]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature( 18,5 ) --> 13
            <span class="cursor-pointer judge-test-button ml-1" data-case="[18,5]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        "#;

        let results = pickup_test_case(html).unwrap();
        let input_cases = get_input_cases(results).unwrap();

        let expecteds = ["3,2", "2,10", "18,5"];

        for (i, actual) in input_cases.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }

    #[test]
    fn test_paser_output_case() {
        let case = "getLowestTemperature(3,2) --> 1";
        let actual = parse_output_case(case).unwrap();

        let expected = "1";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_case() {
        let case = "getLowestTemperature(3,2) --> 1";

        let actual = parse_input_case(case).unwrap();

        let expected = "3,2";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pickup_test_case() {
        let html = r#"
        <p class="m-0 rem0p8">
            getLowestTemperature(3,2) --> 1
            <span class="cursor-pointer judge-test-button ml-1" data-case="[3,2]">
                <i class="far fa-play-circle" title="テスト実行"></i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature(2,10) --> -8
            <span class="cursor-pointer judge-test-button ml-1" data-case="[2,10]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        <p class="m-0 rem0p8">
            getLowestTemperature(18,5) --> 13
            <span class="cursor-pointer judge-test-button ml-1" data-case="[18,5]">
            <i class="far fa-play-circle" title="テスト実行">
            </i>
            </span>
        </p>
        "#;

        let expecteds = vec![
            "getLowestTemperature(3,2) --> 1",
            "getLowestTemperature(2,10) --> -8",
            "getLowestTemperature(18,5) --> 13",
        ];

        let results = pickup_test_case(html).unwrap();

        for (i, actual) in results.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }
}

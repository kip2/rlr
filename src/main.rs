use regex::Regex;
use std::fs;

use scraper::{Html, Selector};

fn main() {
    todo!()
}

fn pickup_test_case(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let p_selector = Selector::parse("p").unwrap();
    let span_selector = Selector::parse("span[data-case]").unwrap();

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
    targets
}

fn get_input_cases(test_cases: Vec<String>) -> Vec<String> {
    test_cases
        .iter()
        .map(|case| parse_input_case(case))
        .collect()
}

fn parse_input_case(test_case: &str) -> String {
    let re = Regex::new(r"\(([^)]+)\)").unwrap();

    let cap = re.captures(&test_case).unwrap();
    let inside = &cap[1];
    inside
        .split(",")
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join(",")
        .to_string()
}

fn get_output_cases(test_cases: Vec<String>) -> Vec<String> {
    test_cases
        .iter()
        .map(|case| parse_output_case(case))
        .collect()
}

fn parse_output_case(test_case: &str) -> String {
    let re = Regex::new(r"--> (.+)$").unwrap();

    let cap = re.captures(&test_case).unwrap();
    let result = cap[1].to_string();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_test_cases() {
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

        let results = pickup_test_case(html);
        let input_cases = get_output_cases(results);

        let expecteds = ["1", "-8", "13"];

        for (i, actual) in input_cases.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }

    #[test]
    fn test_input_test_cases() {
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

        let results = pickup_test_case(html);
        let input_cases = get_input_cases(results);

        let expecteds = ["3,2", "2,10", "18,5"];

        for (i, actual) in input_cases.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }

    #[test]
    fn test_paser_output_case() {
        let case = "getLowestTemperature(3,2) --> 1";
        let actual = parse_output_case(case);

        let expected = "1";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_case() {
        let case = "getLowestTemperature(3,2) --> 1";

        let actual = parse_input_case(case);

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

        let results = pickup_test_case(html);

        for (i, actual) in results.iter().enumerate() {
            assert_eq!(actual, expecteds[i]);
        }
    }
}

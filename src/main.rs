use error::handle_error;
use fetch::fetch_problem_page;
use parser::{get_test_cases, save_test_cases};

mod env;
mod error;
mod fetch;
mod file;
mod parser;

fn main() {
    run();
}

fn run() {
    let url = "https://recursionist.io/dashboard/problems/1";
    download(url);
}

fn download(url: &str) {
    let html = fetch_problem_page(url);

    match get_test_cases(&html) {
        Ok(test_cases) => {
            if let Err(e) = save_test_cases(test_cases) {
                handle_error(e);
            }
        }
        Err(e) => handle_error(e),
    }
}

use crate::file::{get_file_name, read_file};
use colored::Colorize;
use once_cell::sync::Lazy;
use std::{
    fs::{self},
    io::Write,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

pub static SUCCESS_LABEL: Lazy<String> = Lazy::new(|| "SUCCESS".green().to_string());
pub static FAILURE_LABEL: Lazy<String> = Lazy::new(|| "FAILURE".red().to_string());
pub static PASSED_LABEL: Lazy<String> = Lazy::new(|| "passed".green().to_string());
pub static FAILED_LABEL: Lazy<String> = Lazy::new(|| "failed".red().to_string());
pub static INFO_LABEL: Lazy<String> = Lazy::new(|| "INFO".blue().to_string());
pub static AC_LABEL: Lazy<String> = Lazy::new(|| "AC (Accepted)".green().to_string());
pub static WA_LABEL: Lazy<String> = Lazy::new(|| "WA (Wrong Answer)".red().to_string());
pub static RE_LABEL: Lazy<String> = Lazy::new(|| "RE (Runtime Error)".yellow().to_string());
pub static TLE_LABEL: Lazy<String> = Lazy::new(|| "TLE (Time Limit Exceeded)".blue().to_string());
pub static CE_LABEL: Lazy<String> = Lazy::new(|| "CE (Compilation Error)".red().to_string());

#[derive(Debug, PartialEq)]
pub struct TestFile {
    input_file: String,
    output_file: String,
}

impl TestFile {
    fn new(input_file: String, output_file: String) -> Self {
        Self {
            input_file,
            output_file,
        }
    }
}

#[derive(Debug)]
struct JudgeResult {
    case_name: String,
    is_success: bool,
    elapsed_time: Duration,
}

impl JudgeResult {
    fn new(case_name: String, is_success: bool, elapsed_time: Duration) -> Self {
        Self {
            is_success,
            elapsed_time,
            case_name,
        }
    }
}

pub fn judge(command_str: &str) {
    let dir_path = "./testcase";
    let file_list = create_testfile_list(dir_path);
    let version_info = "takerectc 1.0.0";
    let mut slowest_elapsed_time = Duration::new(0, 0);
    let mut slowest_elapsed_case = String::new();

    // start message
    println!("[{}] {}", *INFO_LABEL, version_info);
    println!("[{}] {} cases found", *INFO_LABEL, file_list.len());
    println!("[{}] judge start", *INFO_LABEL);

    println!();
    println!("---------------------------");
    println!();

    // judge
    let total_case = file_list.len();
    let mut success_case = 0;

    for testfile in file_list {
        let input_file_path = testfile.input_file;
        let output_file_path = testfile.output_file;
        let result = judge_test(&input_file_path, &output_file_path, command_str);

        // increment success case count
        if result.is_success {
            success_case += 1;
        }

        // compare elapsed time
        if slowest_elapsed_time < result.elapsed_time {
            slowest_elapsed_time = result.elapsed_time;
            slowest_elapsed_case = result.case_name;
        }
    }

    // end message
    println!("[{}] end judge", *INFO_LABEL);

    println!(
        "[{}] slowest: {:.6} sec (for {})",
        *INFO_LABEL,
        slowest_elapsed_time.as_secs_f64(),
        slowest_elapsed_case
    );

    if success_case == total_case {
        println!(
            "[{}] test {}: {}",
            *SUCCESS_LABEL, *PASSED_LABEL, success_case
        );
    } else if success_case > 0 {
        println!(
            "[{}] test {}: {} | test {}: {}",
            *FAILURE_LABEL,
            *PASSED_LABEL,
            success_case,
            *FAILED_LABEL,
            total_case - success_case
        );
    } else {
        println!(
            "[{}] test {}: {}",
            *FAILURE_LABEL, *FAILED_LABEL, total_case
        );
    }
}

// todo: 未実装箇所あり
fn judge_test(input_path: &str, output_path: &str, command_str: &str) -> JudgeResult {
    // start time measurement
    let start = Instant::now();

    let settion_title = get_file_name(input_path);

    println!("[{}] {}", *INFO_LABEL, settion_title);

    let input_contents = read_file(input_path);
    let output_contents = read_file(output_path);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute on Unix-like system");

    write_to_stdin(&mut child, &input_contents);
    let stdout = child.wait_with_output().unwrap();
    let actual = String::from_utf8_lossy(&stdout.stdout).to_string();

    // todo: execute solve code.

    let duration = start.elapsed();

    println!("[{}] time: {:.6} sec", *INFO_LABEL, duration.as_secs_f64());

    let mut is_success = false;
    if actual.trim() == output_contents.trim() {
        println!("[{}] {}", *SUCCESS_LABEL, *AC_LABEL);
        is_success = true;
    } else {
        println!("[{}] {}", *FAILURE_LABEL, *WA_LABEL);
        println!("input:\n {}", input_contents);
        println!("output:\n {}", actual);
        println!("expected:\n {}", output_contents);
    }

    println!();
    println!("---------------------------");
    println!();

    JudgeResult::new(settion_title.to_string(), is_success, duration)
}

fn write_to_stdin(child: &mut Child, contents: &str) {
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(contents.as_bytes()).unwrap();
}

fn create_testfile_list(path: &str) -> Vec<TestFile> {
    let entries = fs::read_dir(path).unwrap();

    let mut file_list = Vec::<String>::new();
    for entry in entries {
        let entry = entry.expect("error");
        let path = entry.path().to_str().unwrap().to_string();
        file_list.push(path);
    }

    file_list.sort();

    conv_string_to_testfiles(file_list)
}

fn conv_string_to_testfiles(file_list: Vec<String>) -> Vec<TestFile> {
    let mut testfiles_list = Vec::<TestFile>::new();
    for i in (0..file_list.len()).step_by(2) {
        if i + 1 < file_list.len() {
            let input_file = file_list[i].clone();
            let output_file = file_list[i + 1].clone();

            let testfile = TestFile::new(input_file, output_file);
            testfiles_list.push(testfile);
        }
    }

    testfiles_list
}

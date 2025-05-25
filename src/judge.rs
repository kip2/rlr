use crate::error::Error;
use crate::file::{get_file_name, read_file};
use crate::messages::*;
use colored::Colorize;
use std::process::ExitStatus;
use std::{
    fs::{self},
    io::Write,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

#[derive(Debug)]
struct JudgeResult {
    case_name: String,
    is_success: bool,
    elapsed_time: Duration,
}

#[derive(Debug)]
enum Verdict {
    AC,
    WA,
    RE,
    TLE,
}

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

impl JudgeResult {
    fn new(case_name: String, is_success: bool, elapsed_time: Duration) -> Self {
        Self {
            is_success,
            elapsed_time,
            case_name,
        }
    }
}

pub fn judge(command_str: &str) -> Result<(), Error> {
    let dir_path = "./testcase";
    let file_list = create_testfile_list(dir_path)?;
    let version_info = format!("Recursion local runner {}", env!("CARGO_PKG_VERSION"));
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
        let result = judge_test_case(&input_file_path, &output_file_path, command_str)?;

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

    Ok(())
}

fn judge_test_case(
    input_path: &str,
    output_path: &str,
    command_str: &str,
) -> Result<JudgeResult, Error> {
    let timeout = Duration::from_secs(3);

    // start time measurement
    let start = Instant::now();

    let settion_title = get_file_name(input_path)?;

    println!("[{}] {}", *INFO_LABEL, settion_title);

    let input_contents = read_file(input_path)?;
    let output_contents = read_file(output_path)?;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    write_to_stdin(&mut child, &input_contents)?;

    let wait_result = child.wait_timeout(timeout)?;

    let mut actual = String::new();

    let verdict = determine_verdict(child, wait_result, &output_contents, &mut actual)?;

    let duration = start.elapsed();

    let mut is_success = false;

    match verdict {
        Verdict::AC => {
            println!("[{}] time: {:.6} sec", *INFO_LABEL, duration.as_secs_f64());
            println!("[{}] {}", *SUCCESS_LABEL, *AC_LABEL);
            is_success = true;
        }
        Verdict::WA => {
            println!("[{}] time: {:.6} sec", *INFO_LABEL, duration.as_secs_f64());
            println!("[{}] {}", *FAILURE_LABEL, *WA_LABEL);
            println!("input:\n{}", input_contents);
            println!("output:\n{}", actual);
            println!();
            println!("expected:\n{}", output_contents);
        }
        Verdict::RE => {
            println!("[{}] {}", *FAILURE_LABEL, *RE_LABEL);
        }
        Verdict::TLE => {
            println!("[{}] {}", *FAILURE_LABEL, *TLE_LABEL);
            println!(
                "[{}] {}",
                *FAILURE_LABEL,
                "The program ran for more than 3 seconds.".red()
            );
        }
    }

    println!();
    println!("---------------------------");
    println!();

    Ok(JudgeResult::new(
        settion_title.to_string(),
        is_success,
        duration,
    ))
}

fn determine_verdict(
    mut child: Child,
    wait_result: Option<ExitStatus>,
    expected_output: &str,
    actual_output: &mut String,
) -> Result<Verdict, Error> {
    let status = if let Some(status) = wait_result {
        status
    } else {
        let _ = child.kill();
        return Ok(Verdict::TLE);
    };

    if !status.success() {
        return Ok(Verdict::RE);
    }

    let output = child.wait_with_output()?;

    *actual_output = String::from_utf8_lossy(&output.stdout).to_string();
    *actual_output = trim_one_newline(actual_output).to_string();

    if actual_output.trim() == expected_output.trim() {
        Ok(Verdict::AC)
    } else {
        Ok(Verdict::WA)
    }
}

fn trim_one_newline(s: &str) -> &str {
    if s.ends_with('\n') {
        &s[..s.len() - 1]
    } else {
        s
    }
}

fn write_to_stdin(child: &mut Child, contents: &str) -> Result<(), Error> {
    let stdin = child.stdin.as_mut().ok_or(Error::Internal(
        "Failed to stdin in write_to_stdin".to_string(),
    ))?;
    stdin.write_all(contents.as_bytes())?;
    Ok(())
}

fn create_testfile_list(path: &str) -> Result<Vec<TestFile>, Error> {
    let entries = fs::read_dir(path)?;

    let mut file_list = Vec::<String>::new();
    for entry in entries {
        let entry = entry?;
        let path_buf = entry.path();
        let path = path_buf.to_str().ok_or(Error::Internal(
            "Path is not valid UTF-8 in create_testfile_list".to_string(),
        ))?;
        file_list.push(path.to_string());
    }

    file_list.sort();

    Ok(conv_string_to_testfiles(file_list))
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

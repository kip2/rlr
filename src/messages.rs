use colored::Colorize;
use once_cell::sync::Lazy;

pub static SUCCESS_LABEL: Lazy<String> = Lazy::new(|| "SUCCESS".green().to_string());
pub static FAILURE_LABEL: Lazy<String> = Lazy::new(|| "FAILURE".red().to_string());
pub static PASSED_LABEL: Lazy<String> = Lazy::new(|| "passed".green().to_string());
pub static FAILED_LABEL: Lazy<String> = Lazy::new(|| "failed".red().to_string());
pub static INFO_LABEL: Lazy<String> = Lazy::new(|| "INFO".blue().to_string());
pub static NETWORK_LABEL: Lazy<String> = Lazy::new(|| "NETWORK".purple().to_string());
pub static ERROR_LABEL: Lazy<String> = Lazy::new(|| "ERROR".yellow().to_string());
pub static AC_LABEL: Lazy<String> = Lazy::new(|| "AC (Accepted)".green().to_string());
pub static WA_LABEL: Lazy<String> = Lazy::new(|| "WA (Wrong Answer)".red().to_string());
pub static RE_LABEL: Lazy<String> = Lazy::new(|| "RE (Runtime Error)".yellow().to_string());
pub static TLE_LABEL: Lazy<String> = Lazy::new(|| "TLE (Time Limit Exceeded)".yellow().to_string());

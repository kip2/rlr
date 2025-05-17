use colored::Colorize;
use once_cell::sync::Lazy;

pub static SUCCESS_LABEL: Lazy<String> = Lazy::new(|| "SUCCESS".green().to_string());
pub static FAILURE_LABEL: Lazy<String> = Lazy::new(|| "FAILURE".red().to_string());
pub static PASSED_LABEL: Lazy<String> = Lazy::new(|| "PASSED".green().to_string());
pub static FAILED_LABEL: Lazy<String> = Lazy::new(|| "FAILED".red().to_string());
pub static INFO_LABEL: Lazy<String> = Lazy::new(|| "INFO".blue().to_string());
pub static NETWORK_LABEL: Lazy<String> = Lazy::new(|| "NETWORK".purple().to_string());
pub static ERROR_LABEL: Lazy<String> = Lazy::new(|| "ERROR".yellow().to_string());
pub static AC_LABEL: Lazy<String> = Lazy::new(|| "AC (Accepted)".green().to_string());
pub static WA_LABEL: Lazy<String> = Lazy::new(|| "WA (Wrong Answer)".red().to_string());
pub static RE_LABEL: Lazy<String> = Lazy::new(|| "RE (Runtime Error)".yellow().to_string());
pub static TLE_LABEL: Lazy<String> = Lazy::new(|| "TLE (Time Limit Exceeded)".yellow().to_string());

#[cfg(windows)]
pub fn enable_ansi_support() {
    use windows_sys::Win32::System::Console::{
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE,
        SetConsoleMode,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

use std::io;
use std::io::Write;

use clap::Parser;
use clap::Subcommand;
use error::Error;
use error::handle_error;
use file::cookie_path;
use judge::judge;
use regex::Regex;
use request::download;
use request::initial_auth;

mod error;
mod file;
mod judge;
mod messages;
mod parser;
mod request;

#[derive(Parser)]
#[command(
    version,
    about = "rlr はローカルでRecursionの問題を実行するためのツールです",
    long_about = "rlrはローカルでRecursionの問題を実行するためのツールです。\n\
    rlrの簡単な使い方について説明します。\n\
    詳細はGithubページ(https://github.com/kip2/rlr)を参照して下さい。\n\
    \n\
    1. loginオプションを使用して、Recursionへの初回ログインを行って下さい。\n\
    2. downloadオプションで、取得したい問題ページのURLを引数に与え、テストケースの値を取得して下さい。\n\
    3. 好きな言語・好きなエディタで問題を解くコードを書いてください。\n\
    4. 3で書いたコードをシェルから実行するコマンドを用意し、コマンド実行文字列として与えて実行して下さい。 ",
    override_usage = "\
    \n
    login:    rlr login または rlr l
    donwload: rlr download <URL> または rlr d <URL>
    judge:    rlr judge <COMMAND> または rlr j <COMMAND>"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "l", about = "Recursionへのログイン処理を行います。")]
    Login,

    #[command(alias = "d", about = "指定したurlのテストケースをダウンロードします。")]
    Download(DownloadArgs),

    #[command(
        alias = "j",
        about = "カレントディレクトリにあるtestcaseディレクトリに対して、指定されたコマンドを使用してテストを実行します"
    )]
    Judge(JudgeArgs),

    #[command(about = "Cookieファイルの保存パスを取得します。")]
    CookiePath,
}

#[derive(Parser)]
struct JudgeArgs {
    judge_command: String,
}

#[derive(Parser)]
struct DownloadArgs {
    url: String,
}

fn main() {
    #[cfg(windows)]
    {
        use messages::enable_ansi_support;
        enable_ansi_support();
    }

    if let Err(e) = run() {
        handle_error(e);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download(args) => download(&args.url)?,
        Commands::Judge(args) => judge(&args.judge_command)?,
        Commands::Login => login()?,
        Commands::CookiePath => cookie_path()?,
    }

    Ok(())
}

fn login() -> Result<(), Error> {
    let email = prompt_email("Emailアドレス: ")?;

    let password =
        rpassword::prompt_password("Password: ").expect("パスワードの入力処理に失敗しました");

    initial_auth(&email, &password)?;
    Ok(())
}

fn prompt_email(prompt: &str) -> Result<String, Error> {
    loop {
        let mut email = String::new();
        print!("{}", prompt);
        io::stdout().flush()?;
        if let Err(e) = io::stdin().read_line(&mut email) {
            eprintln!("入力に失敗しました: {}", e);
            continue;
        }

        let email = email.trim();

        if valid_email(email)? {
            return Ok(email.to_string());
        }

        println!("Emailの形式で入力して下さい。");
    }
}

fn valid_email(email: &str) -> Result<bool, Error> {
    let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
        .map_err(|_| Error::Internal("Regex compile error in valid_email".to_string()))?;

    Ok(re.is_match(email))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = "email@example.com";

        assert!(valid_email(email).unwrap());
    }

    #[test]
    fn test_valid_email_with_wrong_email() {
        let wrong_email = "email.example.com";
        assert!(!valid_email(wrong_email).unwrap());
    }
}

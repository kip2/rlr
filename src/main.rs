use std::io;
use std::io::Write;

use clap::Parser;
use clap::Subcommand;
use judge::judge;
use regex::Regex;
use request::download;
use request::initial_auth;

mod env;
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
    1. --loginオプションを使用して、Recursionへの初回ログインを行って下さい。\n\
    2. --downloadオプションで、取得したい問題ページのURLを引数に与え、テストケースの値を取得して下さい。\n\
    3. 好きな言語・好きなエディタで問題を解くコードを書いてください。\n\
    4. 3で書いたコードをシェルから実行するコマンドを用意し、コマンド実行文字列として与えて実行して下さい。 "
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "l", about = "Recursionへのログイン処理を行います。")]
    Login,

    #[command(alias = "d", about = "download about")]
    Download(DownloadArgs),

    #[command(alias = "j", about = "judge about")]
    Judge(JudgeArgs),
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Download(args) => download(&args.url),
        Commands::Judge(args) => judge(&args.judge_command),
        Commands::Login => run_login(),
    }
}

fn run_login() {
    let email = prompt_email("Emailアドレス: ");

    let password = rpassword::prompt_password("Password: ").unwrap();

    initial_auth(&email, &password);
}

fn prompt_email(prompt: &str) -> String {
    loop {
        let mut email = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut email).unwrap();
        let email = email.trim().to_string();

        if valid_email(&email) {
            return email;
        }

        println!("Emailの形式で入力して下さい。");
    }
}

fn valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    re.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = "email@example.com";

        assert!(valid_email(email));
    }

    #[test]
    fn test_valid_email_with_wrong_email() {
        let wrong_email = "email.example.com";
        assert!(!valid_email(wrong_email));
    }
}

fn run_judge() {
    let command_str = "bb ./tests/main.clj";
    judge(command_str);
}

fn run() {
    let url = "https://recursionist.io/dashboard/problems/1";
    download(url);
}

use crate::messages::ERROR_LABEL;

type SelectorParseError = Box<dyn std::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Seletor parse error: {0}")]
    Selector(#[from] SelectorParseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("An internal error has occured.\nError: {0})")]
    Internal(String),

    #[error("Failed to get Cookie file path")]
    CookiePathUnvaliable,

    #[error("Network error occurred: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Failed to retrieve cookies after login")]
    CookieMissing,

    #[error("Required header is missing: {0}")]
    HeaderMissing(String),

    #[error("Failed to extract CSRF token from login page: {0}")]
    TokenNotFound(String),

    #[error("Login failed: redirected to unexpected URL")]
    LoginFailed,

    #[error("Malformed cookie: {0}")]
    MalformedCookie(String),

    #[error("No cookie file found")]
    NoCookie,

    #[error("Cookie file is not valid UTF-8")]
    CookieNotUtf8,

    #[error("Incrrect url format")]
    UrlIncorrectFormat,

    #[error("Authentication failed")]
    AuthenticationError,
}

macro_rules! errorln {
    ($($arg:tt)*) => {
        eprintln!("[{}] {}", *ERROR_LABEL, format!($($arg)*) );
    };
}

pub fn handle_error(e: Error) {
    match e {
        Error::Selector(err) => {
            errorln!(
                "テストケースの取得に失敗しました。ログインに失敗しているか、指定したURLが正しくない可能性があります: {}",
                err,
            );
        }
        Error::Io(err) => {
            errorln!("I/O処理でエラーが発生しました: {}", err);
        }
        Error::Internal(err) => {
            errorln!("内部エラーが発生しました。");
            errorln!("{}", err);
        }
        Error::CookiePathUnvaliable => {
            errorln!("Cookieファイルのパス取得に失敗しました。");
        }
        Error::Network(err) => {
            errorln!("ネットワークエラーが発生しました。{}", err);
        }
        Error::CookieMissing => {
            errorln!("Cookieが見つかりません。再度ログインをして下さい");
        }
        Error::HeaderMissing(err) => {
            errorln!(
                "ログイン処理に失敗しました。もう一度やり直すか、開発者にお問い合わせ下さい。"
            );
            errorln!("Error: {}", err);
        }
        Error::TokenNotFound(err) => {
            errorln!(
                "ログイン処理に失敗しました。もう一度やり直すか、開発者にお問い合わせ下さい。"
            );
            errorln!("Error: {}", err);
        }
        Error::LoginFailed => {
            errorln!("ログインに失敗しました。メールアドレスやパスワードをご確認下さい。");
        }
        Error::MalformedCookie(s) => {
            errorln!("Cookieの形式が不正です: {}", s);
        }
        Error::NoCookie => {
            errorln!("ログイン状態の確認に失敗しました。もう一度ログインして下さい。");
        }
        Error::CookieNotUtf8 => {
            errorln!("CookieファイルがUTF-8として正しく読み取れませんでした。");
        }
        Error::UrlIncorrectFormat => {
            errorln!("URLの形式が正しくありません。正しい形式で入力して下さい。");
            errorln!("Example: https://recursionist.io/dashboard/problems/1");
        }
        Error::AuthenticationError => {
            errorln!("認証に失敗しました。ログインし直して下さい。");
        }
    }
}

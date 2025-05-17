type SelectorParseError = Box<dyn std::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Seletor parse error: {0}")]
    Selector(#[from] SelectorParseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("An internal error has occured.\nPlease contact the developer.\nError:({0})")]
    Internal(String),

    #[error("Redirected to unexpected URL: expected {expected}, got {actual}")]
    Unexpectedredirect { expected: String, actual: String },

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

    #[error("")]
    UrlIncorrectFormat,
}

pub fn handle_error(e: Error) {
    match e {
        Error::Selector(err) => {
            eprintln!(
                "テストケースの取得に失敗しました。ログインに失敗しているか、指定したURLが正しくない可能性があります: {}",
                err
            )
        }
        Error::Io(err) => {
            eprintln!("I/O処理でエラーが発生しました: {}", err)
        }
        Error::Internal(err) => {
            eprintln!(
                "内部エラーが発生しました。開発者に連絡して下さい。\nError: {}",
                err
            );
        }
        Error::CookiePathUnvaliable => {
            eprintln!("Cookieファイルのパス取得に失敗しました。")
        }
        Error::Network(err) => {
            eprintln!("ネットワークエラーが発生しました。{}", err);
        }
        Error::Unexpectedredirect { expected, actual } => {
            eprintln!(
                "ページの取得に失敗しました(ページが予期しない場所にリダイレクトされました)。もう一度お試し下さい。\nexpected: {}\nactual: {}",
                expected, actual
            )
        }
        Error::CookieMissing => {
            eprintln!("Cookieが見つかりません。再度ログインをして下さい");
        }
        Error::HeaderMissing(err) => {
            eprintln!(
                "ログイン処理に失敗しました。もう一度やり直すか、開発者にお問い合わせ下さい。\nError: {}",
                err
            )
        }
        Error::TokenNotFound(err) => {
            eprintln!(
                "ログイン処理に失敗しました。もう一度やり直すか、開発者にお問い合わせ下さい。\nError: {}",
                err
            )
        }
        Error::LoginFailed => {
            eprintln!("ログインに失敗しました。メールアドレスやパスワードをご確認下さい。")
        }
        Error::MalformedCookie(s) => {
            eprintln!("Cookieの形式が不正です: {}", s)
        }
        Error::NoCookie => {
            eprintln!("ログイン状態の確認に失敗しました。もう一度ログインして下さい。")
        }
        Error::CookieNotUtf8 => {
            eprintln!("CookieファイルがUTF-8として正しく読み取れませんでした。")
        }
        Error::UrlIncorrectFormat => {
            eprintln!(
                "URLの形式が正しくありません。正しい形式で入力して下さい。\nExample: https://recursionist.io/dashboard/problems/1"
            )
        }
        _ => {
            eprintln!("予期せぬエラーが発生しました。");
        }
    }
}

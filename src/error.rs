type SelectorParseError = Box<dyn std::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Seletor parse error: {0}")]
    Selector(#[from] SelectorParseError),

    #[error("正規表現のコンパイルに失敗しました。: {0}")]
    RegexCompile(#[from] regex::Error),

    #[error("Regex capture failed")]
    RegexCapture,

    #[error("Fale save error: {0}")]
    Io(#[from] std::io::Error),

    #[error("内部エラーが発生しました。開発者に連絡して下さい。")]
    Internal,

    #[error("Failed to determine cookie path")]
    CookiePathMissing,

    #[error("Redirected to unexpected URL: expected {expected}, got {actual}")]
    Unexpectedredirect { expected: String, actual: String },

    #[error("Network error occurred: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Failed to convert header: {0}")]
    HeaderParse(#[from] reqwest::header::ToStrError),

    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Failed to retrieve cookies after login")]
    CookieMissing,

    #[error("Required header is missing")]
    HeaderMissing,

    #[error("Failed to extract CSRF token from login page")]
    TokenNotFound,

    #[error("Login failed: redirected to unexpected URL")]
    LoginFailed,

    // todo: ユーザーに見せるべきかを検討する
    #[error("")]
    FileNameMissing,

    // todo: handle_errorを追加する
    #[error("")]
    FormatMismatch,

    // todo: handle_errorを追加する
    #[error("Malformed cookie: {0}")]
    MalformedCookie(String),

    // todo: handle_errorを追加する
    #[error("Failed to get Cookie file path")]
    CookiePathUnvaliable,
}

pub fn handle_error(e: Error) {
    match e {
        Error::Selector(err) => {
            eprintln!(
                "テストケースのキャプチャに失敗しました。ログインに失敗しているか、取得ページが正しくない可能性があります: {}",
                err
            )
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::RegexCompile(err) => {
            eprintln!("正規表現のコンパイルに失敗しました。")
        }
        Error::RegexCapture => {
            eprintln!("期待されたテストケースの書式が見つかりませんでした。")
        }
        Error::Io(err) => {
            eprintln!("IO処理でエラーが発生しました: {}", err)
        }
        Error::Internal => {
            eprintln!("内部エラーが発生しました。開発者に連絡して下さい。");
        }
        Error::CookiePathMissing => {
            eprintln!("Cookie保存先のパスが見つかりませんでした。");
        }
        // todo: 必要かを検討する
        Error::Unexpectedredirect { expected, actual } => {
            eprintln!("{}{}", expected, actual)
        }
        Error::Network(err) => {
            eprintln!("ネットワークエラーが発生しました。{}", err);
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::HeaderParse(err) => {
            eprintln!("HTMLヘッダー解析に失敗しました。{}", err);
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::UrlParse(err) => {
            eprintln!("URLの解析に失敗しました。{}", err);
        }
        Error::CookieMissing => {
            eprintln!("Cookieが見つかりません。再度ログインをして下さい");
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::HeaderMissing => {
            eprintln!("ヘッダーが見つかりません")
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::TokenNotFound => {
            eprintln!("ログインページにトークンが見つかりませんでした。")
        }
        Error::LoginFailed => {
            eprintln!("ログインに失敗しました。メールアドレスやパスワードをご確認下さい。")
        }
        // todo: ユーザーに見せるべきかを検討する
        Error::FileNameMissing => {
            eprintln!("ファイル名の取得に失敗しました。")
        }
        _ => {
            eprintln!("予期せぬエラーが発生しました。");
        }
    }
}

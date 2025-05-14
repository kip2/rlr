type SelectorParseError = Box<dyn std::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Seletor parse error: {0}")]
    Selector(#[from] SelectorParseError),

    #[error("Failed to compile regex: {0}")]
    RegexCompile(#[from] regex::Error),

    #[error("Regex capture failed")]
    RegexCapture,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("An internal error has occured. Please contact the developer.")]
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

    #[error("File name could not be determined")]
    FileNameMissing,

    #[error("Format does not match expected structure")]
    FormatMismatch,

    #[error("Malformed cookie: {0}")]
    MalformedCookie(String),

    #[error("Failed to get Cookie file path")]
    CookiePathUnvaliable,

    #[error("No cookie file found")]
    NoCookie,

    #[error("Cookie file is not valid UTF-8")]
    CookieNotUtf8,

    #[error("Standard input is unvaliable")]
    StdinUnavailable,

    #[error("Path is not valid UTF-8: {0:?}")]
    NonUtf8Path(std::path::PathBuf),

    #[error("Timed out while waiting for process")]
    WaitTimeoutFailed,
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
        Error::FormatMismatch => {
            eprintln!("ファイルの形式が期待された構造と一致しません")
        }
        Error::MalformedCookie(s) => {
            eprintln!("Cookieの形式が不正です: {}", s)
        }
        Error::CookiePathUnvaliable => {
            eprintln!("Cookieファイルのパス取得に失敗しました。")
        }
        Error::NoCookie => {
            eprintln!("Cookieファイルが見つかりませんでした。")
        }
        Error::CookieNotUtf8 => {
            eprintln!("CookieファイルがUTF-8として正しく読み取れませんでした。")
        }
        Error::StdinUnavailable => {
            eprintln!("標準入力が使用できません。")
        }
        Error::NonUtf8Path(path) => {
            eprintln!("パスがUTF-8として正しくありません: {:?}", path)
        }
        Error::WaitTimeoutFailed => {
            eprintln!("プロセスの終了を待機中にタイムアウトが発生しました。")
        }
        _ => {
            eprintln!("予期せぬエラーが発生しました。");
        }
    }
}

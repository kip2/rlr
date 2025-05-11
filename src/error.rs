type SelectorParseError = Box<dyn std::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Seletor parse error: {0}")]
    Selector(#[from] SelectorParseError),

    #[error("Regex compile error: {0}")]
    RegexCompile(#[from] regex::Error),

    #[error("Regex capture failed")]
    RegexCapture,

    #[error("Fale save error: {0}")]
    Io(#[from] std::io::Error),

    #[error("内部エラーが発生しました。開発者に連絡して下さい。")]
    Internal,

    #[error("Failed to determine cookie path")]
    CookiePathMissing,
}

pub fn handle_error(e: Error) {
    match e {
        Error::RegexCapture => {
            eprintln!("期待されたテストケースの書式が見つかりませんでした。")
        }
        Error::Selector(err) => eprintln!(
            "テストケースのキャプチャに失敗しました。ログインに失敗しているか、取得ページが正しくない可能性があります: {}",
            err
        ),
        Error::Io(err) => {
            eprintln!("ファイルの読み書き中にエラーが発生しました: {}", err)
        }
        Error::Internal => {
            eprintln!("内部エラーが発生しました。開発者に連絡して下さい。");
        }
        _ => {
            eprintln!("予期せぬエラーが発生しました。");
        }
    }
}

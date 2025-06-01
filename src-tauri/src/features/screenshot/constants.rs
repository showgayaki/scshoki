use std::sync::LazyLock;
use std::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub static CANCEL_TOKEN: LazyLock<Mutex<Option<CancellationToken>>> =
    LazyLock::new(|| Mutex::new(None));

pub mod status_messages {
    pub const CREATING_WEBDRIVER: &str = "WebDriverを作成しています...";
    pub const CREATED_WEBDRIVER: &str = "WebDriverを作成しました";
    pub const GETTING_DISPLAY_INFO: &str = "ディスプレイ情報を取得しています...";
    pub const COMBINING: &str = "スクショ画像を結合しています...";

    pub fn capturing(page: &str) -> String {
        format!("{page} のスクショ中です...")
    }
}

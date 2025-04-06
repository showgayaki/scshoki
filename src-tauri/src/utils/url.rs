use crate::config::constants::BROWSER_SCHEMES;

pub fn format_url(url: &str, browser_name: &str) -> String {
    // Safariの場合はURLをそのまま返す
    if browser_name == "safari" {
        return url.to_string();
    }

    // `get()` で URLスキームを取得し、存在しない場合は "https://" を返す
    let scheme = BROWSER_SCHEMES.get(browser_name).unwrap_or({
        // デフォルトのスキームを返す
        &"https://"
    });

    // URLスキームを置き換える
    match browser_name {
        // firefoxの場合はスキームを追加
        "firefox" => format!("{}{}", scheme, url),
        // それ以外は"http(s)://" 部分を置換
        _ => url.replace("https://", scheme).replace("http://", scheme),
    }
}

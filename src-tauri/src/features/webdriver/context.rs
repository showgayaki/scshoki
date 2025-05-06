use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::error::Error;
use std::time::Duration;
use tauri::Url;
use thirtyfour::WebDriver;

use crate::constants::APPIUM_SERVER_URL;
use crate::features::webdriver::services::DriverContext;

/// Appium経由で現在のcontextsを取得（例: ["NATIVE_APP", "WEBVIEW_com.apple.mobilesafari"]）
pub async fn get_contexts(
    session_id: &str,
    appium_url: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    #[derive(Debug, Deserialize)]
    struct ContextsResponse {
        value: Vec<String>,
    }

    let endpoint = format!(
        "{}/session/{}/contexts",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let client = Client::new();
    let response = client.get(&endpoint).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get contexts: {}", response.status()).into());
    }

    let body: ContextsResponse = response.json().await?;

    info!("Available contexts: {:?}", body.value);
    Ok(body.value)
}

/// Appiumでcontext（例: "WEBVIEW_660.4"）を切り替える
pub async fn set_context(
    session_id: &str,
    appium_url: &str,
    context_name: &str,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize)]
    struct SetContextRequest {
        name: String,
    }

    let endpoint = format!(
        "{}/session/{}/context",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let body = SetContextRequest {
        name: context_name.to_string(),
    };
    let response = client.post(&endpoint).json(&body).send().await?;

    if response.status().is_success() {
        debug!("Context set to: {}", context_name);
    } else {
        return Err(format!("Failed to set context: {}", response.status()).into());
    }

    Ok(())
}

/// 指定したURLに一致するWEBVIEW contextを優先的に選び、なければ最大IDのWEBVIEWを返す
pub async fn select_best_context<F, Fut>(
    browser: &str,
    driver: &WebDriver,
    appium_url: &str,
    caps: Map<String, Value>,
    target_url: &Url,
    webdriver: F,
) -> Result<String, String>
where
    F: Fn(Map<String, Value>) -> Fut,
    Fut: std::future::Future<Output = Result<WebDriver, String>> + Send,
{
    debug!("Selecting best context for browser: {}", browser);

    let mut driver = driver.clone();

    let mut session_id = driver.session_id().to_string();
    let contexts = get_contexts(&session_id, appium_url)
        .await
        .map_err(|e| e.to_string())?;
    info!("Available contexts: {:?}", contexts);

    let mut context = "NATIVE_APP".to_string();

    // Firefoxの場合は、同じURLが開かれているタブがあるときに
    // 新しくタブを開かずにそのタブを使用されるため、今回開かれたアクティブなタブを探す
    if browser == "firefox" {
        // コンテキスト切り替えできるものがアクティブ
        for context in contexts {
            if context.starts_with("WEBVIEW_") {
                debug!("Context: {}", context);
                let caps_clone = caps.clone();

                // gotoでページを開いたばっかりなので、set_contextできたタブが
                // テストするページが開かれたタブのはず
                let is_error = match set_context(&session_id, &APPIUM_SERVER_URL, &context).await {
                    Ok(()) => {
                        return Ok(context);
                        // break;
                    }
                    Err(e) => {
                        error!("set_context failed for {}: {}", context, e);
                        true
                    }
                };

                if is_error {
                    // コンテキスト切り替えに失敗した場合は、セッションを削除して再作成
                    debug!("Deleting session: {}", session_id);
                    if let Err(e) = driver.quit().await {
                        error!("Failed to quit driver: {}", e);
                    }
                    let new_driver = webdriver(caps_clone).await?;
                    driver = new_driver;
                    session_id = driver.session_id().to_string();
                    debug!("Sesssion recreated: {}", session_id);

                    // 一度get_contextsを実行しないと、次のset_contextで失敗するっぽい
                    let _ = get_contexts(&session_id, &APPIUM_SERVER_URL).await;

                    continue;
                }
            }
        }
        Ok(context)
    } else {
        // Firefox 以外は常に最大 page_id の WEBVIEW を使用
        contexts
            .iter()
            .filter(|c| c.starts_with("WEBVIEW_"))
            .max_by_key(|c| {
                c.split('.')
                    .nth(1)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0)
            })
            .cloned()
            .ok_or("No valid WEBVIEW context found".to_string())
    }
}

/// デバッグ用に set_context の curl コマンドを出力
pub fn print_set_context_curl(session_id: &str, appium_url: &str, context_name: &str) {
    let url = format!(
        "{}/session/{}/context",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let curl = format!(
        "curl -X POST '{url}' \\\n  -H 'Content-Type: application/json' \\\n  -d '{{\"name\": \"{context}\"}}'",
        url = url,
        context = context_name,
    );

    debug!("\n=== CURL for set_context ===\n{}\n", curl);
}

/// セッションを破棄して再作成し、set_context を再試行する
pub async fn reset_session_and_retry_context<F, Fut>(
    session_id: &str,
    appium_url: &str,
    context_name: &str,
    recreate_session: F,
) -> Result<(), String>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<(String, WebDriver), Box<dyn std::error::Error>>>
        + Send,
{
    // セッション削除
    let delete_url = format!(
        "{}/session/{}",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let res = Client::new()
        .delete(&delete_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Failed to delete session: {}", res.status()));
    }

    // セッション再作成
    let (new_session_id, _driver) = recreate_session().await.map_err(|e| e.to_string())?;

    // Context 再設定
    set_context(&new_session_id, appium_url, context_name)
        .await
        .map_err(|e| e.to_string())?;
    debug!("Context re-set after session reset: {}", context_name);

    Ok(())
}

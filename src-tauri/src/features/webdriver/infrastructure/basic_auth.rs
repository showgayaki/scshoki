use log::{debug, error};
use thirtyfour::prelude::*;

pub async fn submit_basic_auth(
    driver: &WebDriver,
    username: &str,
    password: &str,
) -> WebDriverResult<()> {
    // let source = driver.source().await;
    // debug!("source: {:?}", source);

    // ユーザー名入力欄
    let username_values = ["ユーザ名", "ユーザー名", "ログインID", "ID"];

    let mut username_field_opt = None;
    for value in username_values {
        let selector = format!("//XCUIElementTypeTextField[@value='{}']", value);
        if let Ok(result) = driver.find(By::XPath(selector.clone())).await {
            username_field_opt = Some(result);
            break;
        }
    }

    let username_field = username_field_opt
        .ok_or_else(|| WebDriverError::NotFound("Username field not found".into(), "".into()))?;

    debug!("username_field: {:?}", username_field);
    username_field.send_keys(username).await?;

    // パスワード入力欄
    match driver
        .find(By::XPath("//XCUIElementTypeSecureTextField"))
        .await
    {
        Ok(password_field) => {
            debug!("password_field: {:?}", password_field);
            password_field.send_keys(password).await?;
        }
        Err(e) => {
            error!("Failed to find password field: {:?}", e);
            return Err(e);
        }
    }

    // submitボタン
    // - Safari: サインイン
    // - Chrome: ログイン
    // - Firefox: ログイン
    // - Edge: サインイン
    match driver
        .find(By::XPath(
            "//XCUIElementTypeButton[@label='サインイン' or @label='ログイン']",
        ))
        .await
    {
        Ok(button) => {
            debug!("login_button: {:?}", button);
            button.click().await?;
        }
        Err(e) => {
            error!("Failed to find login button: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}

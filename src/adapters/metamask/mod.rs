use std::collections::HashMap;
use std::fmt::format;
use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;
use url::Url;

use crate::commands::PackageConfig;

pub mod publish;
pub mod unpublish;

pub async fn install_extension(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    phrase: &str,
    password: &str
) -> WebDriverResult<()> {
    // select metamask installation tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    // getting started page
    let content = driver.find_element(By::Id("app-content")).await?;
    let get_started_button = content.find_element(By::ClassName("button")).await?;
    get_started_button.click().await?;

    // import or create new page
    let content = driver.find_element(By::Id("app-content")).await?;
    let import_button = content.find_element(By::ClassName("button")).await?;
    import_button.click().await?;

    // provide feedback opt-in page
    let content = driver.find_element(By::Id("app-content")).await?;
    let no_thanks_button = content.find_element(By::ClassName("button")).await?;
    no_thanks_button.click().await?;

    // import form page
    let content = driver.find_element(By::Id("app-content")).await?;
    let inputs = content.find_elements(By::ClassName("MuiInputBase-input")).await?;
    inputs[0].send_keys(phrase).await?;
    inputs[1].send_keys(password.clone()).await?;
    inputs[2].send_keys(password).await?;
    let terms_checkbox = content.find_element(By::ClassName("first-time-flow__terms")).await?;
    terms_checkbox.click().await?;
    let submit_button = content.find_element(By::ClassName("button")).await?;
    submit_button.click().await?;

    sleep(Duration::from_millis(1000)).await;

    // import results page
    let content = driver.find_element(By::Id("app-content")).await?;
    let ok_button = content.find_element(By::ClassName("button")).await?;
    ok_button.click().await?;

    WebDriverResult::Ok(())
}

pub async fn login(driver: &GenericWebDriver<ReqwestDriverAsync>) -> WebDriverResult<()> {
    // go to OpenSea NFT marketplace login page
    driver.get("https://opensea.io/login").await?;

    // connect compatible wallet
    let metamask_item = driver.find_element(By::XPath("//main//li//button//div//span[contains(text(), 'MetaMask')]")).await?;
    metamask_item.click().await?;

    sleep(Duration::from_millis(5000)).await;

    // select metamask connect window
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[2]).await?;
    // connect metamask wallet
    let next_button = driver.find_element(By::XPath("//button[contains(text(), 'Next')]")).await?;
    next_button.click().await?;
    // connect metamask account
    let connect_button = driver.find_element(By::XPath("//button[contains(text(), 'Connect')]")).await?;
    connect_button.click().await?;
    sleep(Duration::from_millis(5000)).await;

    // select main tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    WebDriverResult::Ok(())
}
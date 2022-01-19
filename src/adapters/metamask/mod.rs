use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;

mod publish;
mod unpublish;
mod reconcile;

pub use publish::publish;
pub use unpublish::unpublish;
pub use reconcile::reconcile;

pub async fn install_extension(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    phrase: &str,
    password: &str
) -> WebDriverResult<()> {
    // select metamask installation tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    // getting started page
    let get_started_button = driver.find_element(By::XPath("//div[contains(@id, 'app-content')]//button")).await?;
    get_started_button.click().await?;

    // import or create new page
    let import_button = driver.find_element(By::XPath("//div[contains(@id, 'app-content')]//button")).await?;
    import_button.click().await?;

    // provide feedback opt-in page
    let no_thanks_button = driver.find_element(By::XPath("//div[contains(@id, 'app-content')]//button")).await?;
    no_thanks_button.click().await?;

    // import form page
    let inputs = driver.find_elements(By::XPath("//div[contains(@id, 'app-content')]//input[contains(concat(' ',@class,' '),' MuiInputBase-input ')]")).await?;
    inputs[0].send_keys(phrase).await?;
    inputs[1].send_keys(password.clone()).await?;
    inputs[2].send_keys(password).await?;
    let terms_checkbox = driver.find_element(By::ClassName("first-time-flow__terms")).await?;
    terms_checkbox.click().await?;
    let submit_button = driver.find_element(By::XPath("//div[contains(@id, 'app-content')]//button")).await?;
    submit_button.click().await?;

    // fixme - without following sleep, selenium gets stale object
    sleep(Duration::from_secs(2)).await;

    // import results page
    let ok_button = driver.find_element(By::XPath("//div[contains(@id, 'app-content')]//button")).await?;
    ok_button.click().await?;

    WebDriverResult::Ok(())
}

pub async fn login(driver: &GenericWebDriver<ReqwestDriverAsync>) -> WebDriverResult<()> {
    // go to OpenSea NFT marketplace login page
    driver.get("https://opensea.io/login").await?;

    // connect compatible wallet
    let metamask_item = driver.find_element(By::XPath("//button//i[contains(text(), 'menu')]")).await?;
    metamask_item.click().await?;

    // connect wallet
    let connect_wallet_button = driver.find_element(By::XPath("//button[contains(text(), 'Connect wallet')]")).await?;
    connect_wallet_button.click().await?;

    // select metamask connect
    let metamask_connect = driver.find_element(By::XPath("//li//button//div//span[contains(text(), 'MetaMask')]")).await?;
    metamask_connect.click().await?;

    // select metamask connect window
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[2]).await?;
    // connect metamask wallet
    let next_button = driver.find_element(By::XPath("//button[contains(text(), 'Next')]")).await?;
    next_button.click().await?;
    // connect metamask account
    let connect_button = driver.find_element(By::XPath("//button[contains(text(), 'Connect')]")).await?;
    connect_button.click().await?;

    // select main tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    WebDriverResult::Ok(())
}
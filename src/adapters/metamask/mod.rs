use tokio::time::{sleep, Duration};
use thirtyfour::{GenericWebDriver, OptionRect};
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;

mod publish;
mod unpublish;
mod reconcile;
mod list;

pub use publish::publish;
pub use unpublish::unpublish;
pub use reconcile::reconcile;
pub use list::list;

pub async fn install_extension(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    phrase: &str,
    password: &str
) -> WebDriverResult<()> {
    // select metamask installation tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    // getting started page
    let get_started_button = driver.find_element(By::XPath("//button[contains(text(), 'Get Started')]")).await?;
    get_started_button.click().await?;

    // import or create new page
    let import_button = driver.find_element(By::XPath("//button[contains(text(), 'Import wallet')]")).await?;
    import_button.click().await?;

    // provide feedback opt-in page
    let no_thanks_button = driver.find_element(By::XPath("//button[contains(text(), 'No Thanks')]")).await?;
    no_thanks_button.click().await?;

    // import form page
    let inputs = driver.find_elements(By::XPath("//input")).await?;
    inputs[0].send_keys(phrase).await?;
    inputs[1].send_keys(password.clone()).await?;
    inputs[2].send_keys(password).await?;
    let terms_checkbox = driver.find_element(By::ClassName("first-time-flow__terms")).await?;
    terms_checkbox.click().await?;
    let submit_button = driver.find_element(By::XPath("//button[contains(text(), 'Import')]")).await?;
    submit_button.click().await?;

    // fixme - without following sleep, selenium gets stale object
    sleep(Duration::from_secs(2)).await;

    // import results page
    let all_done_button = driver.find_element(By::XPath("//button[contains(text(), 'All Done')]")).await?;
    all_done_button.click().await?;

    WebDriverResult::Ok(())
}

pub async fn login(driver: &GenericWebDriver<ReqwestDriverAsync>) -> WebDriverResult<()> {
    let r = OptionRect::new().with_size(1050, 850);
    driver.set_window_rect(r).await?;

    // go to OpenSea NFT marketplace login page
    driver.get("https://opensea.io/").await?;

    let metamask_connect_button = driver.find_element(By::XPath("//button//i[contains(text(), 'menu')]")).await?;
    metamask_connect_button.click().await?;

    let connect_button = driver.find_element(By::XPath("//button[contains(text(), 'Connect wallet')]")).await?;
    connect_button.click().await?;

    sleep(Duration::from_secs(2)).await;

    // select metamask connect
    let metamask_connect = driver.find_element(By::XPath("//span[contains(text(), 'MetaMask')]")).await?;
    metamask_connect.click().await?;

    sleep(Duration::from_secs(2)).await;

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
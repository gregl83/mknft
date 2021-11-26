use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::{SubCommand, ArgMatches, Arg, App};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;

use crate::commands::AttributeValue;
use crate::commands::Attribute;
use crate::commands::ProjectConfig;

use tokio::time::{sleep, Duration};

async fn install_metamask(
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

async fn publish(driver: &GenericWebDriver<ReqwestDriverAsync>) -> WebDriverResult<()> {
    // go to OpenSea NFT marketplace login page (referrer collections)
    driver.get("https://opensea.io/login?referrer=%2Fcollections").await?;

    // connect compatible wallet
    let metamask_item = driver.find_element(By::XPath("//main//li//button//div//span[contains(text(), 'MetaMask')]")).await?;
    metamask_item.click().await?;

    sleep(Duration::from_millis(1000)).await;

    // select metamask connect window
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[2]).await?;

    // connect metamask wallet
    let next_button = driver.find_element(By::XPath("//button[contains(text(), 'Next')]")).await?;
    next_button.click().await?;

    // connect metamask account
    let connect_button = driver.find_element(By::XPath("//button[contains(text(), 'Connect')]")).await?;
    connect_button.click().await?;

    // todo - select collection instead of create

    sleep(Duration::from_millis(60000)).await;

    WebDriverResult::Ok(())
}

pub async fn exec(matches: &ArgMatches<'_>) {
    let selenium_host = matches.value_of("host").unwrap();
    let metamask_crx = matches.value_of("crx").unwrap();
    let metamask_phrase = matches.value_of("phrase").unwrap();
    let metamask_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let mut caps = DesiredCapabilities::chrome();

    caps.add_extension(metamask_crx.as_ref());

    let driver = WebDriver::new(
        format!("http://{}", selenium_host).as_str(),
        &caps
    ).await.unwrap();

    if let Err(e) = install_metamask(
        &driver,
        metamask_phrase,
        metamask_password.as_str()
    ).await {
        println!("MetaMask installation error! {:?}", e);
    } else {
        match publish(&driver).await {
            Ok(_) => println!("done!"),
            Err(e) => println!("Publish error! {:?}", e),
        }
    }

    // cleanup driver connection to selenium chrome
    driver.quit().await.unwrap();
}
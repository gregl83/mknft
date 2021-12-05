use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::{ArgMatches};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;

use crate::commands::PackageConfig;

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

async fn publish(driver: &GenericWebDriver<ReqwestDriverAsync>, package_config: PackageConfig, start: usize, limit: usize) -> WebDriverResult<()> {
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

    // go to create asset to sign request
    driver.get("https://opensea.io/asset/create").await?;
    sleep(Duration::from_millis(5000)).await;

    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[2]).await?;

    let sign = driver.find_element(By::XPath("//button[contains(text(), 'Sign')]")).await?;
    sign.click().await?;
    sleep(Duration::from_millis(5000)).await;

    // select main tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    let collection_asset_create_uri = format!("https://opensea.io/collection/{}/assets/create", package_config.id);
    let images = &package_config.images[start..limit];
    for image in images {
        // create asset
        driver.get(collection_asset_create_uri.as_str()).await?;
        sleep(Duration::from_millis(2000)).await;

        // upload image
        let media_input = driver.find_element(By::XPath("//input[contains(@id, 'media')]")).await?;
        media_input.send_keys(format!("/home/seluser/{}", image.path)).await?;

        // set name
        let name = format!("{} #{}", package_config.name, image.name);
        let name_input = driver.find_element(By::XPath("//input[contains(@id, 'name')]")).await?;
        name_input.send_keys(name.clone()).await?;

        // set description
        let description = format!("**#{}** - {} collection", image.name, package_config.name);
        let description_input = driver.find_element(By::XPath("//textarea[contains(@id, 'description')]")).await?;
        description_input.send_keys(description).await?;

        // set external link
        if let Some(image_uri) = image.uri.clone() {
            let link_input = driver.find_element(By::XPath("//input[contains(@id, 'external_link')]")).await?;
            link_input.send_keys(image_uri).await?;
        }

        // add properties
        let property_button = driver.find_element(By::XPath("//button[contains(@aria-label, 'Add properties')]")).await?;
        property_button.click().await?;
        for (index, property) in image.properties.iter().enumerate() {
            let xpath = format!("//tbody//tr[{}]", index + 1);

            let name_xpath = format!("{}//div[contains(concat(' ',@class,' '),' AssetPropertiesForm--name-input ')]//input", xpath);
            let property_name_input = driver.find_element(By::XPath(name_xpath.as_str())).await?;
            property_name_input.send_keys(package_config.properties[index].clone()).await?;

            let property_xpath = format!("{}//div[contains(concat(' ',@class,' '),' AssetPropertiesForm--value-input ')]//input", xpath);
            let property_value_input = driver.find_element(By::XPath(property_xpath.as_str())).await?;
            property_value_input.send_keys(property).await?;

            if index + 1 < package_config.properties.len() {
                let add_more_button = driver.find_element(By::XPath("//button[contains(text(), 'Add more')]")).await?;
                add_more_button.click().await?;
            }
        }
        let save_properties_button = driver.find_element(By::XPath("//button[contains(text(), 'Save')]")).await?;
        save_properties_button.click().await?;

        // create/mint nft
        let create_button = driver.find_element(By::XPath("//button[contains(text(), 'Create')]")).await?;
        create_button.click().await?;

        // verify complete
        let expected_completion_message = format!("//h4[contains(text(), 'You created {}!')]", name.clone());
        let completion_message = driver.find_element(By::XPath(expected_completion_message.as_str())).await?;
        completion_message.wait_until().displayed().await?;
    }

    WebDriverResult::Ok(())
}

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    let selenium_host = matches.value_of("host").unwrap();
    let metamask_crx = matches.value_of("crx").unwrap();
    let metamask_phrase = matches.value_of("phrase").unwrap();

    let start_arg = matches.value_of("start").unwrap();
    let start = start_arg.parse::<usize>().unwrap();
    let limit_arg = matches.value_of("limit").unwrap();
    let limit = limit_arg.parse::<usize>().unwrap();

    let metamask_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let file = fs::File::open(format!("{}/config.json", src)).expect("file should open read only");
    let package_config: PackageConfig = serde_json::from_reader(file).unwrap();

    let mut caps = DesiredCapabilities::chrome();

    caps.add_extension(metamask_crx.as_ref()).unwrap();

    let driver = WebDriver::new(
        format!("http://{}", selenium_host).as_str(),
        &caps
    ).await.unwrap();

    // set implicit to 10 seconds; default is 0
    driver.set_implicit_wait_timeout(Duration::new(60, 0)).await.unwrap();

    if let Err(e) = install_metamask(
        &driver,
        metamask_phrase,
        metamask_password.as_str()
    ).await {
        println!("MetaMask installation error! {:?}", e);
    } else {
        match publish(&driver, package_config, start, limit).await {
            Ok(_) => println!("done!"),
            Err(e) => println!("Publish error! {:?}", e),
        }
    }

    // cleanup driver connection to selenium chrome
    driver.quit().await.unwrap();
}
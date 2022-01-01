use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;

use crate::commands::PackageConfig;

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

pub async fn publish(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    package_config: PackageConfig,
    start: usize,
    end: usize,
    wait: u64
) -> WebDriverResult<()> {
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

    let mut image_chunks = package_config.images[start..end].chunks(10).peekable();
    while let Some(images) = image_chunks.next() {
        for image in images {
            println!("creating {:?}", image.name.clone());

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

            println!("completed {:?}", image.name.clone());
        }
        if image_chunks.peek().is_some() {
            println!("waiting {:?} seconds", wait);
            sleep(Duration::from_secs(wait)).await;
        }
    }

    WebDriverResult::Ok(())
}

pub async fn unpublish(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    package_config: PackageConfig,
    start: usize,
    end: usize,
    wait: u64
) -> WebDriverResult<()> {
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

    let mut image_chunks = package_config.images[start..end].chunks(10).peekable();
    while let Some(images) = image_chunks.next() {
        for image in images {
            println!("creating {:?}", image.name.clone());

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

            println!("completed {:?}", image.name.clone());
        }
        if image_chunks.peek().is_some() {
            println!("waiting {:?} seconds", wait);
            sleep(Duration::from_secs(wait)).await;
        }
    }

    WebDriverResult::Ok(())
}
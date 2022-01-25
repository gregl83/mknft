use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;
use url::Url;

use crate::commands::PackageConfig;

pub async fn reconcile(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    package_config: PackageConfig,
    start: usize,
    end: usize,
    wait: u64,
    filters: Vec<Vec<String>>
) -> WebDriverResult<()> {
    let collection_asset_create_uri = format!("https://opensea.io/collection/{}/assets/create", package_config.id);
    let collection_uri = format!("https://opensea.io/collection/{}", package_config.id);

    // go to create asset to sign request
    sleep(Duration::from_millis(5000)).await;
    driver.get("https://opensea.io/account").await?;
    sleep(Duration::from_millis(5000)).await;

    let metamask_sign = driver.find_element(By::XPath("//div//i[contains(text(), 'settings')]")).await?;
    metamask_sign.click().await?;

    sleep(Duration::from_millis(5000)).await;

    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[2]).await?;

    let sign = driver.find_element(By::XPath("//button[contains(text(), 'Sign')]")).await?;
    sign.click().await?;
    sleep(Duration::from_millis(5000)).await;

    // select main tab
    let windows = driver.window_handles().await?;
    driver.switch_to().window(&windows[0]).await?;

    let mut counter = 0;
    for image in package_config.images[start..end].iter() {
        let name = format!("{} #{}", package_config.name, image.name);
        let mut query_params: Vec<(String, &str)> = vec![
            (String::from("search[query]"), &name),
            (String::from("search[sortAscending]"), "true"),
            (String::from("search[sortBy]"), "CREATED_DATE")
        ];
        let mut filtered = true;
        for (property_index, filters ) in filters.iter().enumerate() {
            let attribute_name = &package_config.properties[property_index];
            let attribute_value = &image.properties[property_index];
            if !filters.is_empty() && !filters.contains(attribute_value) {
                filtered = false;
                break;
            }
            query_params.push((format!("search[stringTraits][{}][name]", property_index), attribute_name));
            query_params.push((format!("search[stringTraits][{}][values][0]", property_index), attribute_value));
        }
        if !filtered {
            continue;
        }

        println!("reconciling {:?}", image.name.clone());

        let collection_asset_search_uri = Url::parse_with_params(
            collection_uri.as_str(),
            query_params
        ).unwrap();

        driver.get(collection_asset_search_uri.as_str()).await?;

        sleep(Duration::from_millis(3000)).await;

        driver.execute_script(r#"
            window.scrollBy(0,document.body.scrollHeight);
            "#
        ).await?;

        let nft_selector = format!("//img[contains(@alt, '{}')]/ancestor::a[1]", name);
        let nft_href = driver.find_elements(By::XPath(nft_selector.as_str())).await?;
        let mut nft_links: Vec<String> = vec![];

        for nft_link in nft_href.iter() {
            let link = format!("https://opensea.io{}", nft_link.get_attribute("href").await?.unwrap());
            nft_links.push(link);
        }

        match nft_links.len() {
            0 => {
                let not_found = driver.find_element(By::XPath("//button[contains(text(), 'Back to all Items')]")).await;
                if not_found.is_ok() {
                    println!("creating {:?}", image.name.clone());

                    // create asset
                    driver.get(collection_asset_create_uri.as_str()).await?;
                    sleep(Duration::from_millis(2000)).await;

                    // upload image
                    let media_input = driver.find_element(By::XPath("//input[contains(@id, 'media')]")).await?;
                    media_input.send_keys(format!("/home/seluser/collection/{}", image.path)).await?;

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
            },
            1 => {
                continue;
            },
            _ => {
                println!("deleting duplicates of {:?}", image.name.clone());

                for nft_link in nft_links[1..].iter() {
                    driver.get(nft_link).await?;

                    sleep(Duration::from_millis(2000)).await;

                    let edit_button = driver.find_element(By::XPath("//a[contains(text(), 'Edit')]")).await?;
                    edit_button.click().await?;

                    sleep(Duration::from_millis(2000)).await;

                    driver.execute_script(r#"
                        window.scrollBy(0,document.body.scrollHeight);
                        "#
                    ).await?;

                    let delete_button = driver.find_element(By::XPath("//button[contains(text(), 'Delete item')]")).await?;
                    delete_button.click().await?;

                    sleep(Duration::from_millis(2000)).await;

                    let confirm_delete_button = driver.find_element(By::XPath("//footer//button[contains(text(), 'Delete item')]")).await?;
                    confirm_delete_button.click().await?;

                    sleep(Duration::from_millis(2000)).await;
                }
            }
        }

        println!("completed {:?}", image.name.clone());

        counter += 1;
        if counter == 5 {
            println!("waiting {:?} seconds", wait);
            sleep(Duration::from_secs(wait)).await;
            counter = 0;
        }
    }

    WebDriverResult::Ok(())
}
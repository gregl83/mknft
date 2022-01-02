use std::collections::HashMap;
use std::fmt::format;
use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;
use url::Url;

use crate::commands::PackageConfig;

pub async fn unpublish(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    package_config: PackageConfig,
    start: usize,
    end: usize,
    wait: u64,
    filters: Vec<Vec<String>>
) -> WebDriverResult<()> {
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

    let collection_asset_create_uri = format!("https://opensea.io/collection/{}", package_config.id);

    let mut counter = 0;
    for image in package_config.images[start..end].iter() {
        let name = format!("{} #{}", package_config.name, image.name);
        let mut query_params: Vec<(String, &str)> = vec![
            (String::from("search[query]"), &name)
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

        println!("deleting {:?}", image.name.clone());

        let collection_asset_search_uri = Url::parse_with_params(
            collection_asset_create_uri.as_str(),
            query_params

        ).unwrap();

        driver.get(collection_asset_search_uri.as_str()).await?;

        sleep(Duration::from_millis(3000)).await;

        driver.execute_script(r#"
            window.scrollBy(0,document.body.scrollHeight);
            "#
        ).await?;

        let nft_selector = format!("//img[contains(@alt, '{}')]", name);
        if let Ok(nft_image) = driver.find_element(By::XPath(nft_selector.as_str())).await {
            nft_image.click().await?;
        } else {
            continue;
        }

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
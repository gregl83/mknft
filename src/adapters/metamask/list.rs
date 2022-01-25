use tokio::time::{sleep, Duration};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;
use url::Url;

use crate::commands::PackageConfig;

pub async fn list(
    driver: &GenericWebDriver<ReqwestDriverAsync>,
    package_config: PackageConfig,
    start: usize,
    end: usize,
    wait: u64,
    filters: Vec<Vec<String>>
) -> WebDriverResult<()> {
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

        println!("listing {:?}", image.name.clone());

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
        let nft_link = driver.find_element(By::XPath(nft_selector.as_str())).await?;
        let link = format!("https://opensea.io{}", nft_link.get_attribute("href").await?.unwrap());

        driver.get(link).await?;

        sleep(Duration::from_millis(2000)).await;

        let sell_button = driver.find_element(By::XPath("//a[contains(text(), 'Sell')]")).await?;
        sell_button.click().await?;

        let price_input = driver.find_element(By::XPath("//input[@name='price']")).await?;
        price_input.send_keys("0.0045").await?; // fixme - based off calculation

        driver.execute_script("$x(\"//button//div[contains(text(), '1 month')]\").innerHTML = 'February 2, 2022 (12:00 AM) - March 5, 2022 (12:00 AM)';").await?;

        // let complete_listing_button = driver.find_element(By::XPath("//footer//button[contains(text(), 'Complete listing')]")).await?;
        // complete_listing_button.click().await?;

        sleep(Duration::from_millis(60000)).await;

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
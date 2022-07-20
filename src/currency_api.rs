use std::collections::HashMap;
use std::error::Error;

const CURRENCY_API_URL: &str = "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest";

pub async fn fetch_currencies() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let response = reqwest::get(format!("{CURRENCY_API_URL}/currencies.min.json"))
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(response)
}

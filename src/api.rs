use std::collections::HashMap;
use std::error::Error;

pub async fn fetch() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let response = reqwest::get(
        "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies.min.json",
    )
    .await?
    .json::<HashMap<String, String>>()
    .await?;

    Ok(response)
}

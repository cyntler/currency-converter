use std::error::Error;

const CURRENCY_API_URL: &str = "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest";

pub async fn currencies_value(
    from_currency: &String,
    to_currency: &String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let response = reqwest::get(format!(
        "{CURRENCY_API_URL}/currencies/{from_currency}/{to_currency}.min.json"
    ))
    .await?
    .json::<serde_json::Value>()
    .await?;

    Ok(response)
}

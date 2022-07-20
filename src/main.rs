mod currency_api;
use crate::currency_api::fetch_currencies;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let currencies = fetch_currencies().await?;

    println!("{:#?}", currencies);

    Ok(())
}

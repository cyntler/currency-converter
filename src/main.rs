mod api;
use crate::api::fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = fetch().await?;

    println!("{:#?}", resp);
    Ok(())
}

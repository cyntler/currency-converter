use std::env;
use std::error::Error;

use currency_converter::{args_reader, converter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (from_currency, to_currency, amount) = args_reader::read(env::args().skip(1));

    let result = converter::convert(from_currency, to_currency, amount).await;
    println!("{}", result);

    Ok(())
}

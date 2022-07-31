use currency_converter::{args_reader, currency_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (from_currency, to_currency, amount) = args_reader::read();
    let current_currencies_value =
        currency_api::currencies_value(&from_currency, &to_currency).await?;

    let amount_float = amount.parse::<f64>().unwrap();
    let currency_value = current_currencies_value[&to_currency].as_f64().unwrap();

    println!(
        "{:.2} {}",
        amount_float * currency_value,
        to_currency.to_uppercase()
    );

    Ok(())
}

use std::{env, process};

use currency_converter::{args_reader, currency_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (from_currency, to_currency, amount) = args_reader::read(env::args().skip(1));

    let current_currencies_value =
        match currency_api::currencies_value(&from_currency, &to_currency).await {
            Err(_) => {
                eprintln!(
                    "{}",
                    "Currency API does not respond. Check your Internet connection please.",
                );
                process::exit(1);
            }
            Ok(value) => value,
        };

    let amount_float = amount.parse::<f64>().unwrap();
    let currency_value = current_currencies_value[&to_currency].as_f64().unwrap();

    println!(
        "{:.2} {}",
        amount_float * currency_value,
        to_currency.to_uppercase()
    );

    Ok(())
}

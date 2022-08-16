use std::process;

use crate::currency_api;

pub async fn convert(from_currency: String, to_currency: String, amount: String) -> String {
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

    format!(
        "{:.2} {}",
        amount_float * currency_value,
        to_currency.to_uppercase()
    )
}

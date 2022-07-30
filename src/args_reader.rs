use std::env;

pub fn read() -> (String, String, String) {
    let args: Vec<String> = env::args().collect();

    let from_currency = args.get(1).unwrap();
    let to_currency = args.get(2).unwrap();
    let amount = args.get(3).unwrap();

    (
        from_currency.to_string(),
        to_currency.to_string(),
        amount.to_string(),
    )
}

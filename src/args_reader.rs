pub fn read<T: Iterator<Item = String>>(args: T) -> (String, String, String) {
    let args: Vec<String> = args.collect();

    let from_currency = args.get(0).unwrap();
    let to_currency = args.get(1).unwrap();
    let amount = args.get(2).unwrap();

    (
        from_currency.to_string(),
        to_currency.to_string(),
        amount.to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn read_invalid_test() {
        let v = vec!["my".to_string(), "first".to_string()];
        let (arg1, arg2, arg3) = read(v.into_iter());

        assert_eq!(arg1, "my");
        assert_eq!(arg2, "first");
        assert_eq!(arg3, "rusty");
    }

    #[test]
    fn read_valid_test() {
        let v = vec!["my".to_string(), "first".to_string(), "rusty".to_string()];
        let (arg1, arg2, arg3) = read(v.into_iter());

        assert_eq!(arg1, "my");
        assert_eq!(arg2, "first");
        assert_eq!(arg3, "rusty");
    }
}

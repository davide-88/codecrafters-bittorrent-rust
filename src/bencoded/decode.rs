use std::str::FromStr;

fn decode_integer(chars: &str) -> (serde_json::Value, Option<&str>) {
    chars
        .split_once('i')
        .and_then(|(_, interger_with_remainder)| interger_with_remainder.split_once('e'))
        .and_then(|(digits, remainder)| {
            digits
                .parse::<i128>()
                .map(|number| {
                    serde_json::Value::Number(serde_json::Number::from_i128(number).unwrap_or_else(
                        || panic!("{}", format!("Failed to parse integer {digits}")),
                    ))
                })
                .ok()
                .map(|value| (value, Some(remainder)))
        })
        .unwrap_or_else(|| panic!("{}", format!("Failed to find start of integer in {chars}")))
}

fn decode_string(chars: &str) -> (serde_json::Value, Option<&str>) {
    let divider_index = chars
        .find(|char| char == ':')
        .unwrap_or_else(|| panic!("{}", format!("Failed to find string length of {chars}")));
    let length: usize = chars[0..divider_index]
        .parse()
        .unwrap_or_else(|_| panic!("{}", format!("Failed to find string length of {chars}")));
    let end = divider_index + length + 1;
    (
        serde_json::Value::String(
            String::from_str(&chars[divider_index + 1..=divider_index + length]).unwrap_or_else(
                |_| {
                    panic!(
                        "{}",
                        format!("Failed to parse string {}", &chars[divider_index + 1..end])
                    )
                },
            ),
        ),
        chars.get(end..),
    )
}

fn decode_list(chars: &str) -> (serde_json::Value, Option<&str>) {
    let mut list = Vec::new();
    let mut remainder: Option<&str> = chars
        .split_once('l')
        .and_then(|(_, next_chars)| Some(next_chars))
        .filter(|next_chars| next_chars.len() > 0);
    while let Some(next_chars) = remainder.filter(|rem| !rem.starts_with('e')) {
        let (value, rest) = decode_bencoded_value_and_remainder(next_chars);
        remainder = rest;
        list.push(value);
    }
    (
        serde_json::Value::Array(list),
        remainder
            .and_then(|rem| rem.split_once('e'))
            .and_then(|(_, rem)| Some(rem))
            .filter(|rem| rem.len() > 0),
    )
}

fn decode_bencoded_value_and_remainder(encoded_value: &str) -> (serde_json::Value, Option<&str>) {
    let mut chars = encoded_value.chars();
    match chars.nth(0) {
        Some('i') => decode_integer(encoded_value),
        Some('0'..='9') => decode_string(encoded_value),
        Some('l') => decode_list(encoded_value),
        None => (serde_json::Value::Null, None),
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    }
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let decoded = decode_bencoded_value_and_remainder(encoded_value);
    println!("decoded: {:?}", decoded);
    decoded.0
}

#[cfg(test)]
mod tests {
    use super::decode_bencoded_value;

    #[test]
    fn parse_string() {
        let result = decode_bencoded_value(&"5:hello");
        assert_eq!(result, serde_json::Value::String(String::from("hello")));
    }

    #[test]
    fn parse_integer() {
        let result = decode_bencoded_value(&"i52e");
        assert_eq!(
            result,
            serde_json::Value::Number(serde_json::Number::from(52))
        );
    }

    #[test]
    fn parse_list() {
        let result = decode_bencoded_value(&"l5:helloi52ee");
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::String(String::from("hello")),
                serde_json::Value::Number(serde_json::Number::from(52))
            ])
        );
    }

    #[test]
    fn parse_nested_list() {
        let result = decode_bencoded_value(&"l5:helloli52e3:treee");
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::String(String::from("hello")),
                serde_json::Value::Array(vec![
                    serde_json::Value::Number(serde_json::Number::from(52)),
                    serde_json::Value::String(String::from("tre"))
                ]),
            ])
        );
    }
}

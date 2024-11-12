use std::str::FromStr;

fn decode_integer(chars: &str) -> serde_json::Value {
  let end = chars.find(|c| c == 'e')
        .unwrap_or_else(|| panic!("Failed to find end of integer"));
  let raw_int = &chars[1..end];
  println!("raw_int: {}", raw_int);
  serde_json::Value::Number(
    serde_json::Number::from_i128(
      raw_int.parse().expect("Failed to parse integer")
    ).expect("Failed to convert integer to i128")
  )
}

fn decode_string(chars: &str) -> serde_json::Value {
  let divider_index = chars.find(|char| char == ':')
    .unwrap_or_else(|| panic!("Failed to find string length"));
  let length: usize = chars[0..divider_index].parse().expect("Failed to parse string length");
  serde_json::Value::String(
    String::from_str(&chars[divider_index + 1..=divider_index+length])
    .unwrap_or_else(|_| panic!("Failed to parse string"))
  )
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
  let mut chars = encoded_value.chars();
  match chars.nth(0) {
    Some('i') => {
      decode_integer(encoded_value)
    },
    Some('0'..='9') => decode_string(encoded_value),
    None => serde_json::Value::Null,
    _ => panic!("Unhandled encoded value: {}", encoded_value)
  }
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
        assert_eq!(result, serde_json::Value::Number(serde_json::Number::from(52)));
    }
}

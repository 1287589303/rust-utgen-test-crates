// Answer 0

#[test]
fn test_append_pair_with_empty_string() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_separator_needed() {
    let mut string = String::from("existing=pair");
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_custom_encoding() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = Some(&|s| percent_encode_byte(s.as_bytes(), b'=').into_owned());
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_start_position_greater_than_length() {
    let mut string = String::new();
    let start_position = 5;
    let encoding: EncodingOverride = None;
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_large_name_and_value() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "a".repeat(100);
    let value = "b".repeat(100);
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_start_position_equal_to_length() {
    let mut string = String::from("key=value");
    let start_position = string.len();
    let encoding: EncodingOverride = None;
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_100_character_name_and_value() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "n".repeat(100);
    let value = "v".repeat(100);
    append_pair(&mut string, start_position, encoding, name, value);
}

#[test]
fn test_append_pair_with_multiple_encodings() {
    let mut string = String::new();
    let start_position = 0;
    let encoding1: EncodingOverride = Some(&|s| percent_encode_byte(s.as_bytes(), b'%' as u8).into_owned());
    let encoding2: EncodingOverride = Some(&|s| percent_encode_byte(s.as_bytes(), b'=' as u8).into_owned());
    let name = "name";
    let value = "value";
    append_pair(&mut string, start_position, encoding1, name, value);
    append_pair(&mut string, start_position, encoding2, name, value);
}


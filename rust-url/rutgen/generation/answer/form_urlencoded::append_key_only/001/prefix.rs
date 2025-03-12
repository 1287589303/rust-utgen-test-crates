// Answer 0

#[test]
fn test_append_key_only_empty_string() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "key";
    append_key_only(&mut string, start_position, encoding, name);
}

#[test]
fn test_append_key_only_non_empty_string() {
    let mut string = String::from("existing=value");
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "key";
    append_key_only(&mut string, start_position, encoding, name);
}

#[test]
fn test_append_key_only_with_start_position() {
    let mut string = String::from("existing=value");
    let start_position = 15; // start_position equal to string.len()
    let encoding: EncodingOverride = None;
    let name = "key";
    append_key_only(&mut string, start_position, encoding, name);
}

#[test]
fn test_append_key_only_empty_name() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "";
    append_key_only(&mut string, start_position, encoding, name);
}

#[test]
fn test_append_key_only_some_encoding() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = Some(&|s: &str| s.as_bytes().to_vec().into());
    let name = "key";
    append_key_only(&mut string, start_position, encoding, name);
}

#[test]
fn test_append_key_only_special_characters() {
    let mut string = String::new();
    let start_position = 0;
    let encoding: EncodingOverride = None;
    let name = "key with spaces & special#chars!";
    append_key_only(&mut string, start_position, encoding, name);
}


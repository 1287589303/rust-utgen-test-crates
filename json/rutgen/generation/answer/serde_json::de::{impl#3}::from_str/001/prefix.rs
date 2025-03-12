// Answer 0

#[test]
fn test_valid_json_empty_object() {
    let deserializer = Deserializer::from_str("{}");
}

#[test]
fn test_valid_json_empty_array() {
    let deserializer = Deserializer::from_str("[]");
}

#[test]
fn test_valid_json_null() {
    let deserializer = Deserializer::from_str("null");
}

#[test]
fn test_valid_json_true() {
    let deserializer = Deserializer::from_str("true");
}

#[test]
fn test_valid_json_false() {
    let deserializer = Deserializer::from_str("false");
}

#[test]
fn test_valid_json_string() {
    let deserializer = Deserializer::from_str("\"string\"");
}

#[test]
fn test_valid_json_integer() {
    let deserializer = Deserializer::from_str("123");
}

#[test]
fn test_valid_json_float() {
    let deserializer = Deserializer::from_str("123.456");
}

#[test]
fn test_valid_json_object_with_key_value() {
    let deserializer = Deserializer::from_str("{\"key\":\"value\"}");
}

#[test]
fn test_valid_json_array_with_integers() {
    let deserializer = Deserializer::from_str("[1, 2, 3]");
}

#[test]
fn test_invalid_json_missing_quotes() {
    let deserializer = Deserializer::from_str("{key:value}");
}

#[test]
fn test_invalid_json_invalid_integer() {
    let deserializer = Deserializer::from_str("123abc");
}

#[test]
fn test_invalid_json_mismatched_bracket() {
    let deserializer = Deserializer::from_str("[1, 2, 3");
}

#[test]
fn test_invalid_json_empty_string() {
    let deserializer = Deserializer::from_str("");
}


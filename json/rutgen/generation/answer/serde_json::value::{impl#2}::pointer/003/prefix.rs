// Answer 0

#[test]
fn test_pointer_non_empty_non_starting_with_slash_1() {
    let value = Value::Object(Map::new());
    let result = value.pointer("abc");
}

#[test]
fn test_pointer_non_empty_non_starting_with_slash_2() {
    let value = Value::Array(Vec::new());
    let result = value.pointer("xyz");
}

#[test]
fn test_pointer_non_empty_non_starting_with_slash_3() {
    let value = Value::String(String::from("test"));
    let result = value.pointer("123");
}

#[test]
fn test_pointer_non_empty_non_starting_with_slash_4() {
    let value = Value::Bool(true);
    let result = value.pointer("some/pointer");
}


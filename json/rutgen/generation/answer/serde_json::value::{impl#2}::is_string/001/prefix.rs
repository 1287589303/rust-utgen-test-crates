// Answer 0

#[test]
fn test_is_string_with_empty_string() {
    let value = Value::String(String::new());
    let result = value.is_string();
}

#[test]
fn test_is_string_with_non_empty_string() {
    let value = Value::String(String::from("Hello, World!"));
    let result = value.is_string();
}

#[test]
fn test_is_string_with_boolean_true() {
    let value = Value::Bool(true);
    let result = value.is_string();
}

#[test]
fn test_is_string_with_boolean_false() {
    let value = Value::Bool(false);
    let result = value.is_string();
}

#[test]
fn test_is_string_with_integer() {
    let value = Value::Number(Number { n: 42 });
    let result = value.is_string();
}

#[test]
fn test_is_string_with_float() {
    let value = Value::Number(Number { n: 3.14 });
    let result = value.is_string();
}

#[test]
fn test_is_string_with_empty_array() {
    let value = Value::Array(Vec::new());
    let result = value.is_string();
}

#[test]
fn test_is_string_with_non_empty_array() {
    let value = Value::Array(vec![
        Value::String(String::from("item1")),
        Value::Number(Number { n: 1 }),
    ]);
    let result = value.is_string();
}

#[test]
fn test_is_string_with_empty_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let result = value.is_string();
}

#[test]
fn test_is_string_with_populated_object() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let result = value.is_string();
}

#[test]
fn test_is_string_with_null() {
    let value = Value::Null;
    let result = value.is_string();
}

#[test]
fn test_is_string_with_special_characters() {
    let value = Value::String(String::from("!@#$%^&*()_+"));
    let result = value.is_string();
}

#[test]
fn test_is_string_with_long_string() {
    let long_string = String::from("a".repeat(1000)); // 1000 characters long
    let value = Value::String(long_string);
    let result = value.is_string();
}

#[test]
fn test_is_string_with_mixed_object() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert(String::from("string"), Value::String(String::from("text")));
    map.map.insert(String::from("number"), Value::Number(Number { n: 1 }));
    let value = Value::Object(map);
    let result1 = value.is_string(); // Test if the object itself is string
    let result2 = value.as_object().unwrap().get(&String::from("string")).unwrap().is_string(); // Test a specific entry
}

#[test]
fn test_is_string_with_mixed_array() {
    let value = Value::Array(vec![
        Value::String(String::from("text")),
        Value::Number(Number { n: 2 }),
        Value::Bool(false),
    ]);
    let result1 = value.is_string(); // Test if the array itself is string
    let result2 = value.as_array().unwrap()[0].is_string(); // Test the first entry
}


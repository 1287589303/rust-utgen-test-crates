// Answer 0

#[test]
fn test_value_is_null_on_null_variant() {
    let value = Value::Null;
    value.is_null();
}

#[test]
fn test_value_is_null_on_bool_variant() {
    let value_true = Value::Bool(true);
    value_true.is_null();
    let value_false = Value::Bool(false);
    value_false.is_null();
}

#[test]
fn test_value_is_null_on_number_variant_int() {
    let value_int = Value::Number(Number { n: 42 });
    value_int.is_null();
}

#[test]
fn test_value_is_null_on_number_variant_float() {
    let value_float = Value::Number(Number { n: 3.14 });
    value_float.is_null();
}

#[test]
fn test_value_is_null_on_string_variant_empty() {
    let value_empty_string = Value::String(String::new());
    value_empty_string.is_null();
}

#[test]
fn test_value_is_null_on_string_variant_non_empty() {
    let value_non_empty_string = Value::String(String::from("Hello"));
    value_non_empty_string.is_null();
}

#[test]
fn test_value_is_null_on_array_variant_empty() {
    let value_empty_array = Value::Array(Vec::new());
    value_empty_array.is_null();
}

#[test]
fn test_value_is_null_on_array_variant_non_empty() {
    let value_non_empty_array = Value::Array(vec![Value::Null, Value::Bool(true)]);
    value_non_empty_array.is_null();
}

#[test]
fn test_value_is_null_on_object_variant_empty() {
    let value_empty_object = Value::Object(Map { map: MapImpl::new() });
    value_empty_object.is_null();
}

#[test]
fn test_value_is_null_on_object_variant_with_null_and_non_null() {
    let mut object_map = Map { map: MapImpl::new() };
    object_map.insert("a".to_string(), Value::Null);
    object_map.insert("b".to_string(), Value::Bool(false));
    let value_object_with_null = Value::Object(object_map);
    value_object_with_null.is_null();
}


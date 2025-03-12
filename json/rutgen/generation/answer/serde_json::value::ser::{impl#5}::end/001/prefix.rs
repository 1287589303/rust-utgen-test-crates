// Answer 0

#[test]
fn test_end_non_empty_name_and_vec() {
    let name = String::from("test_variant");
    let vec = vec![
        Value::Null,
        Value::Bool(true),
        Value::Number(Number::from(12.5)),
        Value::String(String::from("a string")),
        Value::Array(vec![Value::String(String::from("element"))]),
        Value::Object(Map::new()),
    ];
    let variant = SerializeTupleVariant { name, vec };
    let _result = variant.end();
}

#[test]
fn test_end_with_large_vec_size() {
    let name = String::from("large_variant");
    let vec = (0..10_000).map(|_| Value::String(String::from("large_element"))).collect();
    let variant = SerializeTupleVariant { name, vec };
    let _result = variant.end();
}

#[test]
fn test_end_vec_with_only_null() {
    let name = String::from("null_variant");
    let vec = vec![Value::Null];
    let variant = SerializeTupleVariant { name, vec };
    let _result = variant.end();
}

#[test]
fn test_end_vec_with_one_of_each_type() {
    let name = String::from("mixed_variant");
    let vec = vec![
        Value::Null,
        Value::Bool(false),
        Value::Number(Number::from(42)),
        Value::String(String::from("test")),
        Value::Array(vec![Value::Bool(true)]),
        Value::Object(Map::new()),
    ];
    let variant = SerializeTupleVariant { name, vec };
    let _result = variant.end();
}

#[test]
fn test_end_empty_vec() {
    let name = String::from("empty_variant");
    let vec: Vec<Value> = Vec::new();
    let variant = SerializeTupleVariant { name, vec };
    let _result = variant.end();
}


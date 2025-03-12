// Answer 0

#[test]
fn test_index_into_null() {
    struct Key;

    let key = Key;
    let value = Value::Null;

    let result = (&key).index_into(&value);
}

#[test]
fn test_index_into_bool() {
    struct Key;

    let key = Key;
    let value = Value::Bool(true);

    let result = (&key).index_into(&value);
}

#[test]
fn test_index_into_number() {
    struct Key;

    let key = Key;
    let value = Value::Number(Number::from(42));

    let result = (&key).index_into(&value);
}

#[test]
fn test_index_into_string() {
    struct Key;

    let key = Key;
    let value = Value::String(String::from("test string"));

    let result = (&key).index_into(&value);
}

#[test]
fn test_index_into_array() {
    struct Key;

    let key = Key;
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);

    let result = (&key).index_into(&value);
}


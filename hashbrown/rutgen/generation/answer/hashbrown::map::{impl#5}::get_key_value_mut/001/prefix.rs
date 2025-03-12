// Answer 0

#[test]
fn test_get_key_value_mut_existing_integer_key() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    let (k, v) = map.get_key_value_mut(&1).unwrap();
}

#[test]
fn test_get_key_value_mut_existing_string_key() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), "value");
    let (k, v) = map.get_key_value_mut(&"key".to_string()).unwrap();
}

#[test]
fn test_get_key_value_mut_existing_float_key() {
    let mut map = HashMap::new();
    map.insert(3.14, "pi");
    let (k, v) = map.get_key_value_mut(&3.14).unwrap();
}

#[test]
fn test_get_key_value_mut_existing_tuple_key() {
    let mut map = HashMap::new();
    map.insert((1, 2), "tuple");
    let (k, v) = map.get_key_value_mut(&(1, 2)).unwrap();
}


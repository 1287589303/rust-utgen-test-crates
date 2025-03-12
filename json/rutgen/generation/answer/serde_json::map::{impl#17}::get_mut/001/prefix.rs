// Answer 0

#[test]
fn test_get_mut_with_existing_non_null_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into()), Value::Number(3.0.into())]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            let value_ref = occupied.get_mut();
            // Further operations can be conducted on value_ref if needed
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_get_mut_with_existing_non_null_array_value_and_push() {
    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into()), Value::Number(3.0.into())]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            let value_ref = occupied.get_mut();
            if let Value::Array(arr) = value_ref {
                arr.push(Value::Number(4.0.into()));
            }
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_get_mut_with_existing_value_for_multiple_keys() {
    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into())]));
    map.insert("rust".to_owned(), Value::Array(vec![Value::Number(3.0.into())]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            let value_ref = occupied.get_mut();
            // Potential operations on value_ref
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }

    match map.entry("rust") {
        Entry::Occupied(mut occupied) => {
            let value_ref = occupied.get_mut();
            // Potential operations on value_ref
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}


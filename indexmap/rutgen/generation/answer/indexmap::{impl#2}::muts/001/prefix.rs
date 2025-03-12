// Answer 0

#[test]
fn test_muts_with_integer_key_value() {
    let mut bucket = Bucket {
        hash: HashValue(1),
        key: 42,
        value: 100,
    };
    let (key_mut, value_mut) = bucket.muts();
    *value_mut += 1; // Modifying the mutable value
}

#[test]
fn test_muts_with_string_key_value() {
    let mut bucket = Bucket {
        hash: HashValue(2),
        key: String::from("key"),
        value: String::from("value"),
    };
    let (key_mut, value_mut) = bucket.muts();
    value_mut.push_str(" updated"); // Modifying the mutable value
}

#[test]
fn test_muts_with_boundary_values() {
    let mut bucket = Bucket {
        hash: HashValue(3),
        key: i32::MAX,
        value: 0,
    };
    let (key_mut, value_mut) = bucket.muts();
    *value_mut += 10; // Modifying the mutable value
}

#[test]
fn test_muts_with_empty_string_value() {
    let mut bucket = Bucket {
        hash: HashValue(4),
        key: String::from("empty_key"),
        value: String::new(),
    };
    let (key_mut, value_mut) = bucket.muts();
    value_mut.push('c'); // Modifying the mutable value
}


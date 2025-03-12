// Answer 0

#[test]
fn test_to_vec_pretty_invalid_struct() {
    struct InvalidStruct;

    let result: Result<Vec<u8>> = to_vec_pretty(&InvalidStruct);
}

#[test]
fn test_to_vec_pretty_non_string_map_keys() {
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);

    let result: Result<Vec<u8>> = to_vec_pretty(&map);
}

#[test]
fn test_to_vec_pretty_non_serializable_type() {
    use std::sync::Arc;

    let invalid_data: Arc<i32> = Arc::new(5);

    let result: Result<Vec<u8>> = to_vec_pretty(&invalid_data);
}


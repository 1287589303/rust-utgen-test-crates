// Answer 0

#[test]
fn test_index_mut_with_existing_key() {
    let mut map = Map { map: MapImpl::new() };
    map.map.insert("existing_key".to_string(), Value::String("value".to_string()));
    let index = "existing_key";
    let entry = map.index_mut(&index);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_with_non_existent_key() {
    let mut map = Map { map: MapImpl::new() };
    let index = "non_existent_key";
    let entry = map.index_mut(&index);
}

#[test]
fn test_index_mut_with_empty_map() {
    let mut map = Map { map: MapImpl::new() };
    let index = "key_in_empty_map";
    let entry = map.index_mut(&index);
}


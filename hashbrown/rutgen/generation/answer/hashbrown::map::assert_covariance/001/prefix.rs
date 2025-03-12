// Answer 0

#[test]
fn test_map_key() {
    let map: std::collections::HashMap<&'static str, u8> = [
        ("key1", 1),
        ("key2", 2),
    ].iter().cloned().collect();
    let result: std::collections::HashMap<&str, u8> = map_key(map);
}

#[test]
fn test_map_val() {
    let map: std::collections::HashMap<u8, &'static str> = [
        (1, "value1"),
        (2, "value2"),
    ].iter().cloned().collect();
    let result: std::collections::HashMap<u8, &str> = map_val(map);
}

#[test]
fn test_iter_key() {
    let map: std::collections::HashMap<&'static str, u8> = [
        ("key1", 1),
        ("key2", 2),
    ].iter().cloned().collect();
    let iter = map.iter();
    let result = iter_key(iter);
}

#[test]
fn test_iter_val() {
    let map: std::collections::HashMap<u8, &'static str> = [
        (1, "value1"),
        (2, "value2"),
    ].iter().cloned().collect();
    let iter = map.iter();
    let result = iter_val(iter);
}

#[test]
fn test_into_iter_key() {
    let map: std::collections::HashMap<&'static str, u8> = [
        ("key1", 1),
        ("key2", 2),
    ].iter().cloned().collect();
    let into_iter = map.into_iter();
    let result = into_iter_key(into_iter);
}

#[test]
fn test_into_iter_val() {
    let map: std::collections::HashMap<u8, &'static str> = [
        (1, "value1"),
        (2, "value2"),
    ].iter().cloned().collect();
    let into_iter = map.into_iter();
    let result = into_iter_val(into_iter);
}

#[test]
fn test_keys_key() {
    let map: std::collections::HashMap<&'static str, u8> = [
        ("key1", 1),
        ("key2", 2),
    ].iter().cloned().collect();
    let keys = map.keys();
    let result = keys_key(keys);
}

#[test]
fn test_keys_val() {
    let map: std::collections::HashMap<u8, &'static str> = [
        (1, "value1"),
        (2, "value2"),
    ].iter().cloned().collect();
    let keys = map.keys();
    let result = keys_val(keys);
}

#[test]
fn test_values_key() {
    let map: std::collections::HashMap<&'static str, u8> = [
        ("key1", 1),
        ("key2", 2),
    ].iter().cloned().collect();
    let values = map.values();
    let result = values_key(values);
}

#[test]
fn test_values_val() {
    let map: std::collections::HashMap<u8, &'static str> = [
        (1, "value1"),
        (2, "value2"),
    ].iter().cloned().collect();
    let values = map.values();
    let result = values_val(values);
}

#[test]
fn test_drain() {
    let map: std::collections::HashMap<&'static str, &'static str> = [
        ("key1", "value1"),
        ("key2", "value2"),
    ].iter().cloned().collect();
    let drain = map.clone().drain();
    let result = drain(drain);
}


// Answer 0

#[test]
fn test_retain_in_order_with_non_empty_entries() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(0, 1, "value1".to_string());
    map.push_entry(0, 2, "value2".to_string());
    assert_eq!(map.len(), 2);
    map.retain_in_order(|key, value| {
        *value = format!("updated_{}", value);
        *key % 2 == 0
    });
}

#[test]
fn test_retain_in_order_with_equal_lengths() {
    let mut map: IndexMapCore<String, usize> = IndexMapCore::with_capacity(2);
    map.push_entry(0, "key1".to_string(), 1);
    map.push_entry(0, "key2".to_string(), 2);
    assert_eq!(map.len(), 2);
    map.retain_in_order(|key, value| {
        *value += 1;
        key.len() > 3
    });
}

#[test]
fn test_retain_in_order_with_composite_keys_and_empty_values() {
    let mut map: IndexMapCore<(u32, char), Option<u32>> = IndexMapCore::new();
    map.push_entry(0, (1, 'a'), None);
    map.push_entry(0, (2, 'b'), Some(2));
    assert_eq!(map.len(), 2);
    map.retain_in_order(|key, value| {
        if let Some(v) = value {
            *v = *v + 10;
        }
        key.0 > 1
    });
}

#[test]
fn test_retain_in_order_with_boundary_key_value() {
    let mut map: IndexMapCore<u64, Vec<u8>> = IndexMapCore::new();
    map.push_entry(0, u64::MAX, vec![1, 2, 3]);
    map.push_entry(0, u64::MAX - 1, vec![]);
    assert_eq!(map.len(), 2);
    map.retain_in_order(|key, value| {
        value.push(4);
        *key < u64::MAX
    });
}

#[test]
fn test_retain_in_order_with_custom_retain_logic() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..5 {
        map.push_entry(0, i, i * 10);
    }
    assert_eq!(map.len(), 5);
    map.retain_in_order(|key, value| {
        if *key == 3 {
            *value = 0; // Special case for key 3
            true
        } else {
            *key != 0
        }
    });
}


// Answer 0

#[test]
fn test_or_insert_with_key_occupied_case() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("hello", 5);

    match map.entry("hello") {
        Entry::Occupied(mut entry) => {
            let value = entry.or_insert_with_key(|key| {
                assert_eq!(key, &"hello");
                10
            });
            // This should mutate the existing value
            *value *= 2; // Existing value of 5, should become 10
        }
        _ => unreachable!(),
    }

    // Validate that value has been updated correctly
    assert_eq!(map["hello"], 10);
}


#[test]
fn test_or_insert_with_key_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("world", 3);

    match map.entry("world") {
        Entry::Occupied(mut entry) => {
            let value = entry.or_insert_with_key(|key| {
                assert_eq!(key, &"world");
                7
            });
            // Mutate the existing value
            *value += 2; // Existing value of 3, should become 5
        }
        _ => unreachable!(),
    }

    // Validate that value has been updated correctly
    assert_eq!(map["world"], 5);
}


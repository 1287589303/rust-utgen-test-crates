// Answer 0

#[test]
fn test_replace_entry_with_removes_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("test_key", 42);

    let entry = match map.entry("test_key") {
        Entry::Occupied(e) => e.replace_entry_with(|k, v| {
            assert_eq!(k, &"test_key");
            assert_eq!(v, 42);
            None
        }),
        Entry::Vacant(_) => panic!(),
    };

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"test_key");
        }
        Entry::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("test_key"));
}

#[test]
fn test_replace_entry_with_replaces_value() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("another_key", 100);

    let entry = match map.entry("another_key") {
        Entry::Occupied(e) => e.replace_entry_with(|k, v| {
            assert_eq!(k, &"another_key");
            assert_eq!(v, 100);
            Some(v + 1)
        }),
        Entry::Vacant(_) => panic!(),
    };

    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"another_key");
            assert_eq!(e.get(), &101);
        }
        Entry::Vacant(_) => panic!(),
    }

    assert_eq!(map["another_key"], 101);
}


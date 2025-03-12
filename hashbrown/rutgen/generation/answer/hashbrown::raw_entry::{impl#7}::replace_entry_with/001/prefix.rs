// Answer 0

#[test]
fn test_replace_entry_with_should_remain_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            Some(v + 1)
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let (key, value) = o.get_key_value();
            assert_eq!(key, &"a");
            assert_eq!(value, &101);
        }
    }
}

#[test]
fn test_replace_entry_with_multiple_calls_should_remain_occupied_first_call() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("b", 200);

    let raw_entry_first = match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"b");
            assert_eq!(v, 200);
            Some(v + 100)
        }),
    };

    assert!(matches!(raw_entry_first, RawEntryMut::Occupied(_)));

    let raw_entry_second = match raw_entry_first {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"b");
            assert_eq!(v, 300);
            Some(v + 50)
        }),
    };

    match raw_entry_second {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let (key, value) = o.get_key_value();
            assert_eq!(key, &"b");
            assert_eq!(value, &350);
        }
    }
}


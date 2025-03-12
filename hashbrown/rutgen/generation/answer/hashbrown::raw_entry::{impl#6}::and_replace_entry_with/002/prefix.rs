// Answer 0

#[test]
fn test_and_replace_entry_with_existing_entry_returning_some() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = map
        .raw_entry_mut()
        .from_key("poneyland")
        .and_replace_entry_with(|k, v| {
            assert_eq!(k, &"poneyland");
            assert_eq!(v, 42);
            Some(v + 1)
        });

    match entry {
        RawEntryMut::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43);
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_replace_entry_with_existing_entry_returning_none() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = map
        .raw_entry_mut()
        .from_key("poneyland")
        .and_replace_entry_with(|_k, _v| None);

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("poneyland"));
}

#[test]
#[should_panic]
fn test_and_replace_entry_with_non_existent_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("non_existent")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}


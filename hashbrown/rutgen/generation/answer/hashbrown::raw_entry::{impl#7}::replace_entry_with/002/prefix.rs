// Answer 0

#[test]
fn replace_entry_with_removes_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|_k, _v| None),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };
}

#[test]
fn replace_entry_with_removes_entry_multiple_keys() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("x", 300);
    map.insert("y", 400);

    let raw_entry = match map.raw_entry_mut().from_key(&"x") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|_k, _v| None),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };
    assert_eq!(map.get(&"x"), None);
}


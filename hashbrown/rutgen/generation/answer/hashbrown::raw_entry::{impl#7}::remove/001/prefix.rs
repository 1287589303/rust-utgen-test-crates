// Answer 0

#[test]
fn test_remove_first_element() {
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let entry = map.raw_entry_mut().from_key(&"a").unwrap();
    entry.remove();
}

#[test]
fn test_remove_last_element() {
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let entry = map.raw_entry_mut().from_key(&"b").unwrap();
    entry.remove();
}

#[test]
fn test_remove_only_element() {
    let mut map: HashMap<&str, u32> = [("a", 100)].into();
    let entry = map.raw_entry_mut().from_key(&"a").unwrap();
    entry.remove();
}

#[test]
fn test_remove_nonexistent_element() {
    let mut map: HashMap<&str, u32> = [("a", 100)].into();
    let entry = map.raw_entry_mut().from_key(&"b");
    match entry {
        RawEntryMut::Vacant(_) => {}
        RawEntryMut::Occupied(_) => panic!(),
    }
}


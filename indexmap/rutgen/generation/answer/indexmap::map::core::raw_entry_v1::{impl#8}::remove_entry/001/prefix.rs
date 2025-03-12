// Answer 0

#[test]
fn test_remove_entry_from_empty_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    entry.remove_entry();
}

#[test]
fn test_remove_entry_from_single_entry_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, String::from("one"));
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    entry.remove_entry();
}

#[test]
fn test_remove_entry_from_multiple_entries_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(1),
        hash_builder: PhantomData,
    };
    entry.remove_entry();
}

#[test]
fn test_remove_entry_at_last_index() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(1),
        hash_builder: PhantomData,
    };
    entry.remove_entry();
}

#[test]
fn test_remove_entry_from_full_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, format!("value {}", i));
    }
    
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(500),
        hash_builder: PhantomData,
    };
    entry.remove_entry();
}


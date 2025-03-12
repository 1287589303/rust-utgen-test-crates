// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);

    let entry_ref = EntryRef::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket::new((String::from("poneyland"), 3)),
        table: &mut map,
    });

    let key = entry_ref.key();
    let key_borrowed: &str = key.borrow();
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, u32> = HashMap::new();

    let entry_ref = EntryRef::Vacant(VacantEntryRef {
        hash: 0,
        key: &"horseland",
        table: &mut map,
    });

    let key = entry_ref.key();
    let key_borrowed: &str = key.borrow();
}


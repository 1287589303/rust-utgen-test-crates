// Answer 0

#[test]
fn test_entry_key_occupied() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    let index = hashbrown::hash_table::OccupiedEntry::from_index(0);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    let entry = Entry::Occupied(occupied_entry);

    let key = entry.key();
}

#[test]
fn test_entry_key_vacant() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::new(),
        key: TestKey,
    };
    let entry = Entry::Vacant(vacant_entry);

    let key = entry.key();
}


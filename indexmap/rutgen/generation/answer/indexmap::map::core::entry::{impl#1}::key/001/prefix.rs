// Answer 0

#[test]
fn test_key_vacant_entry() {
    struct TestKey;
    struct TestValue;

    let key = TestKey;
    let value = TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(),
        key,
    };

    let entry = Entry::Vacant(vacant_entry);
    let _key_ref = entry.key();
}

#[test]
fn test_key_vacant_entry_with_non_empty_map() {
    struct TestKey;
    struct TestValue;

    let key = TestKey;
    let value = TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    entries.insert(key, value);

    let vacant_key = TestKey;
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(),
        key: vacant_key,
    };

    let entry = Entry::Vacant(vacant_entry);
    let _key_ref = entry.key();
}


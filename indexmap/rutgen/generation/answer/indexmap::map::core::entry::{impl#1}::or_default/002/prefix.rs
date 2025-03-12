// Answer 0

#[test]
fn test_or_default_occupied_entry() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: 0 }
        }
    }

    let mut entries: Entries<TestKey, TestValue> = Entries::new();
    let key = TestKey;
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    let entry = Entry::Occupied(occupied_entry);
    
    let value_ref = entry.or_default();
}

#[test]
fn test_or_default_vacant_entry() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: 0 }
        }
    }

    let mut entries: Entries<TestKey, TestValue> = Entries::new();
    let key = TestKey;
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(),
        key,
    };
    let entry = Entry::Vacant(vacant_entry);

    let value_ref = entry.or_default();
}


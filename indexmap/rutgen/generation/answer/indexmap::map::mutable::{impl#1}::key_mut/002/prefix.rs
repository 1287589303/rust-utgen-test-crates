// Answer 0

#[test]
fn test_key_mut_occupied() {
    struct TestKey;
    struct TestValue;

    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
        fn push(&mut self, key: K, value: V) {
            self.data.push((key, value));
        }
    }

    let mut entries = TestEntries::<TestKey, TestValue>::new();
    entries.push(TestKey, TestValue);
    
    let index_value = 0; 
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(index_value));

    let entry = Entry::Occupied(occupied_entry);
    let key_mut_ref = entry.key_mut();
}

#[test]
fn test_key_mut_vacant() {
    struct TestKey;
    struct TestValue;

    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
        fn push(&mut self, key: K, value: V) {
            self.data.push((key, value));
        }
    }

    let mut entries = TestEntries::<TestKey, TestValue>::new();
    let vacant_entry = VacantEntry::new(&mut entries, HashValue::default(), TestKey);
    
    let entry = Entry::Vacant(vacant_entry);
    let key_mut_ref = entry.key_mut();
}


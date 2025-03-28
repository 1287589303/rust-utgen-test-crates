// Answer 0

#[test]
fn test_or_insert_with_occupied() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    struct TestKey(usize);
    struct TestValue(String);

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let hash_builder = RandomState::new();
    let mut map: IndexMap<TestKey, TestValue, _> = IndexMap::with_hasher(hash_builder);
    
    map.insert(TestKey(1), TestValue("Occupied".to_string()));
    
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries(),
        index: map.get_index_of(&TestKey(1)).unwrap(), // Assuming this correctly fetches an index
        hash_builder: PhantomData,
    });

    let result = occupied_entry.or_insert_with(|| (TestKey(2), TestValue("New".to_string())));
    
    // Call the function with a closure.
    let _key_ref = result.0;
    let _value_ref = result.1;
}

#[test]
fn test_or_insert_with_vacant() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    
    struct TestKey(usize);
    struct TestValue(String);

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let hash_builder = RandomState::new();
    let mut map: IndexMap<TestKey, TestValue, _> = IndexMap::with_hasher(hash_builder);
    
    // Inserting a key to ensure the next call can find a vacant entry.
    map.insert(TestKey(1), TestValue("Occupied".to_string()));

    // Using a RawEntryMut::Vacant by targeting a non-existent key.
    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &hash_builder,
    });
    
    let result = vacant_entry.or_insert_with(|| (TestKey(2), TestValue("Inserted".to_string())));
    
    let _key_ref = result.0;
    let _value_ref = result.1;
}


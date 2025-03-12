// Answer 0

#[test]
fn test_shift_remove_full_valid_entry() {
    struct KeyWrapper(usize);
    
    impl Equivalent<usize> for KeyWrapper {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }
    
    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(1);
    
    index_map.entries.push(Bucket {
        hash: hash_value,
        key: 1,
        value: "value1",
    });
    
    index_map.indices.insert(hash_value.get(), 0); // Simulating that an entry exists
    
    let key_ref = &KeyWrapper(1);
    
    let result = index_map.shift_remove_full(hash_value, key_ref);
}

#[test]
fn test_shift_remove_full_entry_at_end() {
    struct KeyWrapper(usize);
    
    impl Equivalent<usize> for KeyWrapper {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }
    
    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(2);
    
    index_map.entries.push(Bucket {
        hash: hash_value,
        key: 2,
        value: "value2",
    });
    index_map.indices.insert(hash_value.get(), 0);
    
    let key_ref = &KeyWrapper(2);
    
    let result = index_map.shift_remove_full(hash_value, key_ref);
}

#[test]
fn test_shift_remove_full_multiple_entries() {
    struct KeyWrapper(usize);
    
    impl Equivalent<usize> for KeyWrapper {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }
    
    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(3);
    
    index_map.entries.push(Bucket { hash: hash_value, key: 3, value: "value3" });
    index_map.entries.push(Bucket { hash: hash_value, key: 4, value: "value4" });
    index_map.indices.insert(hash_value.get(), 0);
    
    let key_ref = &KeyWrapper(3);
    
    let result = index_map.shift_remove_full(hash_value, key_ref);
} 

#[test]
fn test_shift_remove_full_with_collision() {
    struct KeyWrapper(usize);
    
    impl Equivalent<usize> for KeyWrapper {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }
    
    let mut index_map = IndexMapCore::new();
    let hash_value1 = HashValue(5);
    let hash_value2 = HashValue(5); // Same hash for collision simulation
    
    index_map.entries.push(Bucket { hash: hash_value1, key: 5, value: "value5" });
    index_map.entries.push(Bucket { hash: hash_value2, key: 6, value: "value6" });
    index_map.indices.insert(hash_value1.get(), 0);
    
    let key_ref = &KeyWrapper(5);
    
    let result = index_map.shift_remove_full(hash_value1, key_ref);
}


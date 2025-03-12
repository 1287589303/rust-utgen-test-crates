// Answer 0

#[test]
fn test_insert_with_occupied_entry() {
    struct TestHasher;
    
    impl Default for TestHasher {
        fn default() -> Self {
            TestHasher
        }
    }

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, i32, TestHasher> = HashMap::new();
    map.insert("key1", 10);
    let entry = map.entry("key1");

    let occupied_entry = entry.insert(); // Will match Entry::Occupied
}

#[test]
fn test_insert_with_multiple_occupied_entries() {
    struct TestHasher;

    impl Default for TestHasher {
        fn default() -> Self {
            TestHasher
        }
    }

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<&str, i32, TestHasher> = HashMap::new();
    map.insert("keyA", 20);
    map.insert("keyB", 30);
    let entry_a = map.entry("keyA");
    let entry_b = map.entry("keyB");

    let occupied_entry_a = entry_a.insert(); // Matches Entry::Occupied
    let occupied_entry_b = entry_b.insert(); // Matches Entry::Occupied
}

#[test]
fn test_insert_with_occupied_entry_and_multiple_types() {
    struct TestHasher;

    impl Default for TestHasher {
        fn default() -> Self {
            TestHasher
        }
    }

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: HashMap<i32, String, TestHasher> = HashMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    let entry_1 = map.entry(1);
    let entry_2 = map.entry(2);

    let occupied_entry_1 = entry_1.insert(); // Will match Entry::Occupied
    let occupied_entry_2 = entry_2.insert(); // Will match Entry::Occupied
}


// Answer 0

#[test]
fn test_append_with_non_empty_sets() {
    struct DummyHasher; // This is a simple struct to serve as the hasher.
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut self_set = IndexSet::<i32, DummyHasher>::from([1, 2, 3]);
    let mut other_set = IndexSet::<i32, DummyHasher>::from([2, 3, 4, 5]);

    self_set.append(&mut other_set);
}

#[test]
fn test_append_self_with_duplicates() {
    struct DummyHasher; 
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut self_set = IndexSet::<i32, DummyHasher>::from([3, 3, 1]); // Duplicates in self
    let mut other_set = IndexSet::<i32, DummyHasher>::from([3, 4, 5]);

    self_set.append(&mut other_set);
}

#[test]
fn test_append_empty_other_set() {
    struct DummyHasher; 
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut self_set = IndexSet::<i32, DummyHasher>::from([1, 2, 3]);
    let mut other_set = IndexSet::<i32, DummyHasher>::from([]); // Empty other set

    self_set.append(&mut other_set);
}

#[test]
fn test_append_with_identical_elements() {
    struct DummyHasher; 
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut self_set = IndexSet::<i32, DummyHasher>::from([1, 2, 3]);
    let mut other_set = IndexSet::<i32, DummyHasher>::from([1, 2, 3]); // Identical elements in other

    self_set.append(&mut other_set);
}


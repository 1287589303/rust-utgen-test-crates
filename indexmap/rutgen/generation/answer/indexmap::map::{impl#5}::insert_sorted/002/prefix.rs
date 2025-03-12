// Answer 0

#[test]
fn test_insert_sorted_existing_key() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, _: &[u8]) {}
    }

    impl BuildHasher for TestHasher {
        type Hasher = Self;
        fn build_hasher(&self) -> Self {
            TestHasher
        }
    }

    let mut map = IndexMap::<i32, String, TestHasher>::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let result = map.insert_sorted(2, "updated two".to_string());
}

#[test]
fn test_insert_sorted_new_key() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, _: &[u8]) {}
    }

    impl BuildHasher for TestHasher {
        type Hasher = Self;
        fn build_hasher(&self) -> Self {
            TestHasher
        }
    }

    let mut map = IndexMap::<i32, String, TestHasher>::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let result = map.insert_sorted(4, "four".to_string());
}

#[test]
fn test_insert_sorted_edge_case() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, _: &[u8]) {}
    }

    impl BuildHasher for TestHasher {
        type Hasher = Self;
        fn build_hasher(&self) -> Self {
            TestHasher
        }
    }

    let mut map = IndexMap::<i32, String, TestHasher>::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let result = map.insert_sorted(1, "updated one".to_string());
}


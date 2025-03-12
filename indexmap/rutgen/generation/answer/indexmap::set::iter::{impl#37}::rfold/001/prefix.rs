// Answer 0

#[test]
fn test_rfold_with_integer_initial_value() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let init: i32 = 0;
    let f = |acc: i32, _: &i32| acc + 1;
    
    let set1: IndexSet<i32, DummyHasher> = IndexSet::new(); // Initialize as empty
    let set2: IndexSet<i32, DummyHasher> = IndexSet::new(); // Initialize as empty
    
    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let result = symmetric_difference.rfold(init, f);
}

#[test]
fn test_rfold_with_string_initial_value() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let init: String = String::from("Start");
    let f = |acc: String, item: &String| acc + item;

    let mut set1: IndexSet<String, DummyHasher> = IndexSet::new();
    set1.insert(String::from("A"));
    set1.insert(String::from("B"));

    let mut set2: IndexSet<String, DummyHasher> = IndexSet::new();
    set2.insert(String::from("B"));
    set2.insert(String::from("C"));

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let result = symmetric_difference.rfold(init, f);
}

#[test]
fn test_rfold_with_large_collection() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let init: usize = 0;
    let f = |acc: usize, _: &usize| acc + 1;

    let mut set1: IndexSet<usize, DummyHasher> = IndexSet::new();
    for i in 1..1000 {
        set1.insert(i);
    }

    let mut set2: IndexSet<usize, DummyHasher> = IndexSet::new();
    for i in 500..1500 {
        set2.insert(i);
    }

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let result = symmetric_difference.rfold(init, f);
}

#[test]
fn test_rfold_with_empty_collections() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let init: i32 = 42;
    let f = |acc: i32, _: &i32| acc + 1;

    let set1: IndexSet<i32, DummyHasher> = IndexSet::new(); // Initialize as empty
    let set2: IndexSet<i32, DummyHasher> = IndexSet::new(); // Initialize as empty

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let result = symmetric_difference.rfold(init, f);
}


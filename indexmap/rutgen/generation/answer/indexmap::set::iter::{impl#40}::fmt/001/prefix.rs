// Answer 0

#[test]
fn test_symmetric_difference_debug_empty() {
    struct FakeHasher;
    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: IndexSet<i32, FakeHasher> = IndexSet::new();
    let set2: IndexSet<i32, FakeHasher> = IndexSet::new();
    
    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };
    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_difference = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let _ = format!("{:?}", symmetric_difference);
}

#[test]
fn test_symmetric_difference_debug_non_empty() {
    struct FakeHasher;
    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set1: IndexSet<i32, FakeHasher> = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, FakeHasher> = IndexSet::new();
    set2.insert(2);
    set2.insert(3);

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };
    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_difference = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let _ = format!("{:?}", symmetric_difference);
}

#[test]
fn test_symmetric_difference_debug_with_duplicates() {
    struct FakeHasher;
    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: IndexSet<i32, FakeHasher> = {
        let mut s = IndexSet::new();
        s.insert(1);
        s.insert(3);
        s
    };

    let set2: IndexSet<i32, FakeHasher> = {
        let mut s = IndexSet::new();
        s.insert(3);
        s.insert(4);
        s
    };

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };
    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_difference = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let _ = format!("{:?}", symmetric_difference);
}


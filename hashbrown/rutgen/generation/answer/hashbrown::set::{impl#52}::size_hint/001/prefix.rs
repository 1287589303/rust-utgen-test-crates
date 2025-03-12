// Answer 0

#[test]
fn test_size_hint_non_empty() {
    use std::collections::HashSet;
    use std::hash::{BuildHasher, Hasher};

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let set_a: HashSet<i32, DummyHasher> = {
        let mut h = HashSet::with_capacity(3);
        h.insert(1);
        h.insert(2);
        h;
    };
    
    let set_b: HashSet<i32, DummyHasher> = {
        let mut h = HashSet::with_capacity(2);
        h.insert(2);
        h.insert(3);
        h;
    };

    let diff = Difference { iter: set_a.iter(), other: &set_b };
    let sym_diff = SymmetricDifference { iter: diff.chain(set_b.iter()) };

    let (lower, upper) = sym_diff.size_hint();
    println!("Size hint: {:?}, {:?}", lower, upper);
}

#[test]
fn test_size_hint_empty_sets() {
    use std::collections::HashSet;
    use std::hash::{BuildHasher, Hasher};

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let set_a: HashSet<i32, DummyHasher> = HashSet::new();
    let set_b: HashSet<i32, DummyHasher> = HashSet::new();

    let diff = Difference { iter: set_a.iter(), other: &set_b };
    let sym_diff = SymmetricDifference { iter: diff.chain(set_b.iter()) };

    let (lower, upper) = sym_diff.size_hint();
    println!("Size hint: {:?}, {:?}", lower, upper);
}

#[test]
fn test_size_hint_varied_sizes() {
    use std::collections::HashSet;
    use std::hash::{BuildHasher, Hasher};

    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let set_a: HashSet<i32, DummyHasher> = {
        let mut h = HashSet::with_capacity(4);
        h.insert(1);
        h.insert(2);
        h.insert(3);
        h.insert(4);
        h;
    };
    
    let set_b: HashSet<i32, DummyHasher> = {
        let mut h = HashSet::with_capacity(1);
        h.insert(4);
        h;
    };

    let diff = Difference { iter: set_a.iter(), other: &set_b };
    let sym_diff = SymmetricDifference { iter: diff.chain(set_b.iter()) };

    let (lower, upper) = sym_diff.size_hint();
    println!("Size hint: {:?}, {:?}", lower, upper);
}


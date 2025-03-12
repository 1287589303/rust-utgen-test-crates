// Answer 0

#[test]
fn test_intersection_next_no_matches() {
    struct BuildHasherImpl;
    impl BuildHasher for BuildHasherImpl {
        type Hasher = std::hash::BuildHasherDefault<core::hash::VHasher>;
        fn build_hasher(&self) -> Self::Hasher {
            self.default()
        }
    }

    let set1 = IndexSet {
        map: IndexMap::from_iter(vec!["apple", "banana"]),
    };
    let set2 = IndexSet {
        map: IndexMap::from_iter(vec!["orange", "grape"]),
    };

    let mut iter = Iter {
        iter: set1.iter(),
    };

    let mut intersection = Intersection {
        iter,
        other: &set2,
    };

    let result = intersection.next();
}

#[test]
fn test_intersection_next_multiple_items_no_matches() {
    struct BuildHasherImpl;
    impl BuildHasher for BuildHasherImpl {
        type Hasher = std::hash::BuildHasherDefault<core::hash::VHasher>;
        fn build_hasher(&self) -> Self::Hasher {
            self.default()
        }
    }

    let set1 = IndexSet {
        map: IndexMap::from_iter(vec!["dog", "cat", "mouse"]),
    };
    let set2 = IndexSet {
        map: IndexMap::from_iter(vec!["hamster", "guinea pig"]),
    };

    let mut iter = Iter {
        iter: set1.iter(),
    };

    let mut intersection = Intersection {
        iter,
        other: &set2,
    };

    let result1 = intersection.next();
    let result2 = intersection.next();
    let result3 = intersection.next();
}


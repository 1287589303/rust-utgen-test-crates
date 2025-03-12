// Answer 0

#[test]
fn test_difference_next_returns_item_not_in_other() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let item_not_in_other = "test_item";
    let other_set: IndexSet<&str, TestHasher> = IndexSet::new(); // other set is empty

    let buckets = vec![Bucket::new(item_not_in_other)];
    let iter = SliceIter::new(&buckets);

    let mut difference = Difference {
        iter: Iter { iter },
        other: &other_set,
    };

    let result = difference.next();
}

#[test]
fn test_difference_next_with_multiple_items() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let item_not_in_other = "unique_item";
    let other_set: IndexSet<&str, TestHasher> = IndexSet::new(); // other set is empty

    let buckets = vec![Bucket::new("common_item"), Bucket::new(item_not_in_other)];
    let iter = SliceIter::new(&buckets);

    let mut difference = Difference {
        iter: Iter { iter },
        other: &other_set,
    };

    let result = difference.next();
}

#[test]
fn test_difference_next_with_empty_other() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let item_not_in_other = "item_not_in_other";
    let other_set: IndexSet<&str, TestHasher> = IndexSet::new(); // other set is empty

    let buckets = vec![Bucket::new(item_not_in_other)];
    let iter = SliceIter::new(&buckets);

    let mut difference = Difference {
        iter: Iter { iter },
        other: &other_set,
    };

    let result = difference.next();
}


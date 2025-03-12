// Answer 0

#[test]
fn test_next_back_with_matching_items() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let other_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap::from_iter(vec![(1, ()), (2, ()), (3, ())]),
    };
    
    let buckets = vec![
        Bucket { key: 3, value: () },
        Bucket { key: 2, value: () },
        Bucket { key: 1, value: () },
    ];

    let iter = Iter {
        iter: buckets.iter(),
    };

    let mut difference = Difference { iter, other: &other_set };
    
    assert!(difference.next_back().is_some());
    assert!(difference.next_back().is_some());
    assert!(difference.next_back().is_some());
    let result = difference.next_back();
    assert!(result.is_none());
}


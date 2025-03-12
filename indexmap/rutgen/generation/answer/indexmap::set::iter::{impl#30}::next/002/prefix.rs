// Answer 0

#[test]
fn test_intersection_next_with_common_item() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    struct TestBucket {
        item: i32,
    }

    impl Eq for TestBucket {}

    impl Hash for TestBucket {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.item.hash(state);
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap::new(),
    };

    index_set.insert(TestBucket { item: 1 });
    index_set.insert(TestBucket { item: 2 });

    let buckets = vec![Bucket { item: TestBucket { item: 1 } }];
    let iter = Iter {
        iter: buckets.iter(),
    };

    let mut intersection = Intersection {
        iter,
        other: &index_set,
    };

    let result = intersection.next();
    // No assertion, as per the instructions.
}

#[test]
fn test_intersection_next_with_multiple_common_items() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    struct TestBucket {
        item: i32,
    }

    impl Eq for TestBucket {}

    impl Hash for TestBucket {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.item.hash(state);
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap::new(),
    };

    index_set.insert(TestBucket { item: 1 });
    index_set.insert(TestBucket { item: 2 });
    index_set.insert(TestBucket { item: 3 });

    let buckets = vec![
        Bucket { item: TestBucket { item: 1 } },
        Bucket { item: TestBucket { item: 2 } },
    ];
    
    let iter = Iter {
        iter: buckets.iter(),
    };

    let mut intersection = Intersection {
        iter,
        other: &index_set,
    };

    let result = intersection.next();
    // No assertion, as per the instructions.
}

#[test]
fn test_intersection_next_with_no_common_items() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    struct TestBucket {
        item: i32,
    }

    impl Eq for TestBucket {}

    impl Hash for TestBucket {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.item.hash(state);
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap::new(),
    };

    index_set.insert(TestBucket { item: 3 });
    index_set.insert(TestBucket { item: 4 });

    let buckets = vec![Bucket { item: TestBucket { item: 1 } }];
    
    let iter = Iter {
        iter: buckets.iter(),
    };

    let mut intersection = Intersection {
        iter,
        other: &index_set,
    };

    let result = intersection.next();
    // No assertion, as per the instructions.
}


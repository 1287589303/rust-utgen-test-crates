// Answer 0

#[test]
fn test_next_back_non_empty() {
    struct TestHasher;
    // Implement required BuildHasher trait methods here

    let hasher1 = TestHasher;
    let hasher2 = TestHasher;

    let set1 = IndexSet::<u32, TestHasher>::from_iter(vec![1, 2, 3]);
    let set2 = IndexSet::<u32, TestHasher>::from_iter(vec![3, 4, 5]);

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };

    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_diff = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let mut iter = symmetric_diff;
    let _ = iter.next_back();
}

#[test]
fn test_next_back_single_element() {
    struct TestHasher;
    // Implement required BuildHasher trait methods here

    let hasher1 = TestHasher;
    let hasher2 = TestHasher;

    let set1 = IndexSet::<u32, TestHasher>::from_iter(vec![1]);
    let set2 = IndexSet::<u32, TestHasher>::from_iter(vec![1]);

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };

    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_diff = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let mut iter = symmetric_diff;
    let _ = iter.next_back();
}

#[test]
fn test_next_back_one_empty() {
    struct TestHasher;
    // Implement required BuildHasher trait methods here
    
    let hasher1 = TestHasher;
    let hasher2 = TestHasher;

    let set1 = IndexSet::<u32, TestHasher>::from_iter(vec![1, 2, 3]);
    let set2 = IndexSet::<u32, TestHasher>::new();

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };

    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_diff = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let mut iter = symmetric_diff;
    let _ = iter.next_back();
}

#[test]
fn test_next_back_large_collection() {
    struct TestHasher;
    // Implement required BuildHasher trait methods here

    let hasher1 = TestHasher;
    let hasher2 = TestHasher;

    let set1 = IndexSet::<u32, TestHasher>::from_iter((1..1000).collect::<Vec<u32>>());
    let set2 = IndexSet::<u32, TestHasher>::from_iter((500..1500).collect::<Vec<u32>>());

    let difference1 = Difference {
        iter: set1.iter(),
        other: &set2,
    };

    let difference2 = Difference {
        iter: set2.iter(),
        other: &set1,
    };

    let symmetric_diff = SymmetricDifference {
        iter: difference1.chain(difference2),
    };

    let mut iter = symmetric_diff;
    while let Some(_) = iter.next_back() {}
}


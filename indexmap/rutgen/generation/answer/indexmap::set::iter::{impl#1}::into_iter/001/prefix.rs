// Answer 0

#[test]
fn test_into_iter_with_non_empty_index_set() {
    struct HashBuilder;

    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: "value1" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 2, value: "value2" };
    let buckets = vec![bucket1, bucket2];

    let index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(buckets.clone()),
            hash_builder: HashBuilder,
        },
    };

    let iter = index_set.into_iter();
}

#[test]
fn test_into_iter_with_single_entry_index_set() {
    struct HashBuilder;

    let bucket = Bucket { hash: HashValue::default(), key: 1, value: "single_value" };
    let buckets = vec![bucket];

    let index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(buckets.clone()),
            hash_builder: HashBuilder,
        },
    };

    let iter = index_set.into_iter();
}

#[test]
fn test_into_iter_boundary_case() {
    struct HashBuilder;

    let bucket = Bucket { hash: HashValue::default(), key: 0, value: "boundary_value" };
    let buckets = vec![bucket];

    let index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(buckets.clone()),
            hash_builder: HashBuilder,
        },
    };

    let iter = index_set.into_iter();
}


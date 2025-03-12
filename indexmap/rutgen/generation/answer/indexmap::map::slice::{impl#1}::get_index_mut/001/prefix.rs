// Answer 0

#[test]
fn test_get_index_mut_valid_indices() {
    struct TestKey;
    struct TestValue;
    
    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        ],
    };

    let index_0 = slice.get_index_mut(0);
    let index_last = slice.get_index_mut(slice.len() - 1);
}

#[test]
fn test_get_index_mut_out_of_bounds() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        ],
    };

    let index_out_of_bounds = slice.get_index_mut(slice.len());
    let index_out_of_bounds_plus_one = slice.get_index_mut(slice.len() + 1);
}

#[test]
fn test_get_index_mut_empty_slice() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice::new_mut();

    let index_empty = slice.get_index_mut(0);
}


// Answer 0

#[test]
fn test_index_mut_valid_index() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: "a", value: 1 },
            Bucket { hash: 1, key: "b", value: 2 },
        ],
    };
    let index = 0;
    let value_ref = slice.index_mut(index);
}

#[test]
fn test_index_mut_valid_index_boundary() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: "a", value: 1 },
            Bucket { hash: 1, key: "b", value: 2 },
        ],
    };
    let index = 1;
    let value_ref = slice.index_mut(index);
}

#[test]
#[should_panic]
fn test_index_mut_invalid_index_too_low() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: "a", value: 1 },
        ],
    };
    let index = usize::MAX; // Invalid index to trigger panic
    let value_ref = slice.index_mut(index);
}

#[test]
#[should_panic]
fn test_index_mut_invalid_index_too_high() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: "a", value: 1 },
        ],
    };
    let index = 1; // Out of bounds index
    let value_ref = slice.index_mut(index);
}


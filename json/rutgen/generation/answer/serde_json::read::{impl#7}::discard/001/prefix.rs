// Answer 0

#[test]
fn test_discard_increments_index_on_non_empty_slice() {
    let mut slice_read = SliceRead {
        slice: &[1, 2, 3, 4, 5],
        index: 2,
    };
    slice_read.discard();
}

#[test]
fn test_discard_increments_index_at_boundary() {
    let mut slice_read = SliceRead {
        slice: &[1, 2, 3],
        index: 0,
    };
    slice_read.discard();
}

#[test]
fn test_discard_on_last_index() {
    let mut slice_read = SliceRead {
        slice: &[1],
        index: 0,
    };
    slice_read.discard();
}

#[test]
fn test_discard_on_empty_slice() {
    let mut slice_read = SliceRead {
        slice: &[],
        index: 0,
    };
    slice_read.discard();
}


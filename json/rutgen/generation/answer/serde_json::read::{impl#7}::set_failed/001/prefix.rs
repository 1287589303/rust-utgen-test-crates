// Answer 0

#[test]
fn test_set_failed_index_zero() {
    let mut failed = false;
    let slice = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 0 };

    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_index_length() {
    let mut failed = false;
    let slice = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: slice.len() };

    reader.set_failed(&mut failed);
}

#[test]
fn test_set_failed_index_middle() {
    let mut failed = false;
    let slice = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 2 };

    reader.set_failed(&mut failed);
}


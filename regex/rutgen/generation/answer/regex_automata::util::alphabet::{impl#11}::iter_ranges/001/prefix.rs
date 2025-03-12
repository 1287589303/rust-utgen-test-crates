// Answer 0

#[test]
fn test_iter_ranges_empty() {
    let byte_set = ByteSet::empty();
    let iter = byte_set.iter_ranges();
}

#[test]
fn test_iter_ranges_single_byte() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(5);
    let iter = byte_set.iter_ranges();
}

#[test]
fn test_iter_ranges_multiple_contiguous_bytes() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(1);
    byte_set.add(2);
    byte_set.add(3);
    let iter = byte_set.iter_ranges();
}

#[test]
fn test_iter_ranges_non_contiguous_bytes() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(1);
    byte_set.add(3);
    byte_set.add(5);
    let iter = byte_set.iter_ranges();
}


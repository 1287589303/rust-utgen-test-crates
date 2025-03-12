// Answer 0

#[test]
fn test_set_range_start_equals_end() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(0, 0);
}

#[test]
fn test_set_range_start_equals_end_non_zero() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(10, 10);
}

#[test]
fn test_set_range_start_equals_end_at_max() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(255, 255);
}

#[test]
fn test_set_range_start_zero_end_non_zero() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(0, 1);
}

#[test]
fn test_set_range_start_zero_end_at_max() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(0, 255);
}


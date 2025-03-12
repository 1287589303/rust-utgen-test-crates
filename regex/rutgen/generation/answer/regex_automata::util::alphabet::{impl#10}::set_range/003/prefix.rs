// Answer 0

#[test]
#[should_panic]
fn test_set_range_start_greater_than_end() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(5, 3);
}

#[test]
#[should_panic]
fn test_set_range_start_equals_end_boundary() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(1, 0);
}

#[test]
#[should_panic]
fn test_set_range_start_equals_end_non_boundary() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(10, 5);
}

#[test]
#[should_panic]
fn test_set_range_start_higher_end() {
    let mut byte_class_set = ByteClassSet::empty();
    byte_class_set.set_range(255, 200);
}


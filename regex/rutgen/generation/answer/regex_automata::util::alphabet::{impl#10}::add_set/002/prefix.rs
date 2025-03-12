// Answer 0

#[test]
fn test_add_set_empty() {
    let mut byte_class_set = ByteClassSet::empty();
    let byte_set = ByteSet::empty();
    byte_class_set.add_set(&byte_set);
}

#[test]
fn test_add_set_single_value() {
    let mut byte_class_set = ByteClassSet::empty();
    let mut byte_set = ByteSet::empty();
    byte_set.add(10);
    byte_class_set.add_set(&byte_set);
}

#[test]
fn test_add_set_no_ranges() {
    let mut byte_class_set = ByteClassSet::empty();
    let mut byte_set = ByteSet::empty();
    
    byte_set.add(0);
    byte_set.add(1);
    byte_set.add(2);
    byte_set.add(3);
    
    byte_class_set.add_set(&byte_set);
}

#[test]
fn test_add_set_min_value() {
    let mut byte_class_set = ByteClassSet::empty();
    let mut byte_set = ByteSet::empty();
    
    byte_set.add(0);
    byte_class_set.add_set(&byte_set);
}

#[test]
fn test_add_set_max_value() {
    let mut byte_class_set = ByteClassSet::empty();
    let mut byte_set = ByteSet::empty();
    
    byte_set.add(255);
    byte_class_set.add_set(&byte_set);
}

#[test]
fn test_add_set_non_contiguous() {
    let mut byte_class_set = ByteClassSet::empty();
    let mut byte_set = ByteSet::empty();
    
    byte_set.add(5);
    byte_set.add(20);
    byte_set.add(35);
    byte_class_set.add_set(&byte_set);
}


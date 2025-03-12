// Answer 0

#[test]
fn test_add_valid_byte_0() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
}

#[test]
fn test_add_valid_byte_127() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(127);
}

#[test]
fn test_add_valid_byte_255() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(255);
}

#[test]
fn test_add_no_op_byte_0() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
    byte_set.add(0);
}

#[test]
fn test_add_no_op_byte_127() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(127);
    byte_set.add(127);
}

#[test]
fn test_add_no_op_byte_255() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(255);
    byte_set.add(255);
}

#[test]
fn test_add_multiple_bytes() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
    byte_set.add(127);
    byte_set.add(255);
}


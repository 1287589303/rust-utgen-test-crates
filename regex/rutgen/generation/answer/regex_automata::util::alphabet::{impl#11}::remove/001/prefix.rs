// Answer 0

#[test]
fn test_remove_byte_present_0() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
    byte_set.remove(0);
}

#[test]
fn test_remove_byte_present_127() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(127);
    byte_set.remove(127);
}

#[test]
fn test_remove_byte_present_128() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(128);
    byte_set.remove(128);
}

#[test]
fn test_remove_byte_present_255() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(255);
    byte_set.remove(255);
}

#[test]
fn test_remove_byte_not_present_0() {
    let mut byte_set = ByteSet::empty();
    byte_set.remove(0);
}

#[test]
fn test_remove_byte_not_present_127() {
    let mut byte_set = ByteSet::empty();
    byte_set.remove(127);
}

#[test]
fn test_remove_byte_not_present_128() {
    let mut byte_set = ByteSet::empty();
    byte_set.remove(128);
}

#[test]
fn test_remove_byte_not_present_255() {
    let mut byte_set = ByteSet::empty();
    byte_set.remove(255);
}


// Answer 0

#[test]
fn test_write_to_len() {
    let byte_set = ByteSet::default();
    let result = byte_set.write_to_len();
}

#[test]
fn test_write_to_len_non_empty() {
    let mut byte_set = ByteSet::default();
    byte_set.add(0);
    let result = byte_set.write_to_len();
} 

#[test]
fn test_write_to_len_multiple_bytes() {
    let mut byte_set = ByteSet::default();
    byte_set.add(1);
    byte_set.add(2);
    byte_set.add(3);
    let result = byte_set.write_to_len();
}


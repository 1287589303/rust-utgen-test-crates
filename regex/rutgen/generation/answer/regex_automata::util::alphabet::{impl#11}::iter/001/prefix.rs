// Answer 0

#[test]
fn test_iter_empty_set() {
    let empty_set = ByteSet::empty();
    let iter = empty_set.iter();
}

#[test]
fn test_iter_single_byte() {
    let mut single_byte_set = ByteSet::empty();
    single_byte_set.add(0x00);
    let iter = single_byte_set.iter();
}

#[test]
fn test_iter_multiple_bytes() {
    let mut multiple_bytes_set = ByteSet::empty();
    multiple_bytes_set.add(0x00);
    multiple_bytes_set.add(0x7F);
    multiple_bytes_set.add(0xFF);
    let iter = multiple_bytes_set.iter();
}

#[test]
fn test_iter_max_size_set() {
    let mut max_size_set = ByteSet::empty();
    for byte in 0..=0xFF {
        max_size_set.add(byte);
    }
    let iter = max_size_set.iter();
}


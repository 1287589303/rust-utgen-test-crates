// Answer 0

#[test]
fn test_keep_last_bytes_zero_len() {
    let mut seq = Seq::new(&[b"a", b"foo", b"quux"]);
    seq.keep_last_bytes(0);
}

#[test]
fn test_keep_last_bytes_less_than_literal_length() {
    let mut seq = Seq::new(&[b"hello", b"world", b"rust"]);
    seq.keep_last_bytes(3);
}

#[test]
fn test_keep_last_bytes_equal_to_literal_length() {
    let mut seq = Seq::new(&[b"byte", b"code", b"test"]);
    seq.keep_last_bytes(4);
}

#[test]
fn test_keep_last_bytes_more_than_literal_length() {
    let mut seq = Seq::new(&[b"short", b"longer", b"lengthy"]);
    seq.keep_last_bytes(10);
}


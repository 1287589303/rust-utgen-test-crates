// Answer 0

#[test]
fn test_keep_first_bytes_with_exact_literals() {
    let mut seq = Seq::new(&[b"abc", b"defg", b"hijkl"]);
    seq.keep_first_bytes(2);
}

#[test]
fn test_keep_first_bytes_with_mixed_length_literals() {
    let mut seq = Seq::new(&[b"x", b"yzabc", b"foo", b"quuxxx"]);
    seq.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_when_len_equals_literal_length() {
    let mut seq = Seq::new(&[b"short", b"exact", b"match"]);
    seq.keep_first_bytes(6);
}

#[test]
fn test_keep_first_bytes_when_len_is_zero() {
    let mut seq = Seq::new(&[b"nonempty", b"literals", b"here"]);
    seq.keep_first_bytes(0);
}

#[test]
fn test_keep_first_bytes_with_literally_empty_literals() {
    let mut seq = Seq::new(&[b"", b"nonempty"]);
    seq.keep_first_bytes(1);
}


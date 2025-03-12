// Answer 0

#[test]
fn test_split_to_empty_string() {
    let mut b1 = Bytes::from("");
    _split_to_must_use();
}

#[test]
fn test_split_to_full_string() {
    let mut b1 = Bytes::from("hello world");
    b1.split_to(11);
}

#[test]
fn test_split_to_boundary_case_zero() {
    let mut b1 = Bytes::from("hello");
    b1.split_to(0);
}

#[test]
fn test_split_to_boundary_case_length_one() {
    let mut b1 = Bytes::from("hello");
    b1.split_to(1);
}

#[test]
fn test_split_to_boundary_case_length_ten() {
    let mut b1 = Bytes::from("hello world");
    b1.split_to(10);
}

#[test]
fn test_split_to_boundary_case_length_nine() {
    let mut b1 = Bytes::from("hello world");
    b1.split_to(9);
}

#[test]
fn test_split_to_boundary_case_length_eleven() {
    let mut b1 = Bytes::from("hello world");
    b1.split_to(11);
}


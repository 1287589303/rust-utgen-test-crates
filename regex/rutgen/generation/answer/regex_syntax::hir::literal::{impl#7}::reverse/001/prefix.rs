// Answer 0

#[test]
fn test_reverse_single_byte() {
    let mut literal = Literal::exact(vec![1]);
    literal.reverse();
}

#[test]
fn test_reverse_two_bytes() {
    let mut literal = Literal::exact(vec![1, 2]);
    literal.reverse();
}

#[test]
fn test_reverse_multiple_bytes() {
    let mut literal = Literal::exact(vec![1, 2, 3, 4, 5]);
    literal.reverse();
}

#[test]
fn test_reverse_large_bytes() {
    let mut literal = Literal::exact((0..1000).map(|x| x as u8).collect::<Vec<u8>>());
    literal.reverse();
}

#[test]
fn test_reverse_edge_case_empty_bytes() {
    let mut literal = Literal::exact(vec![0]);
    literal.reverse();
}

#[test]
fn test_reverse_large_capacity() {
    let mut literal = Literal::exact(vec![255; 1000]);
    literal.reverse();
}


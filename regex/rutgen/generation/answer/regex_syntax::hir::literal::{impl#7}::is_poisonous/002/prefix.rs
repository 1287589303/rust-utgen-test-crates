// Answer 0

#[test]
fn test_is_poisonous_with_two_bytes_high_rank() {
    let mut literal = Literal::exact(vec![255, 1]);
    let result = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_with_two_bytes_max_byte() {
    let mut literal = Literal::exact(vec![250, 2]);
    let result = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_with_varied_bytes_high_rank() {
    let mut literal = Literal::exact(vec![251, 3]);
    let result = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_with_multiple_bytes_high_rank() {
    let mut literal = Literal::exact(vec![200, 200]);
    let result = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_with_first_byte_rank_255() {
    let mut literal = Literal::exact(vec![255, 255]);
    let result = literal.is_poisonous();
}


// Answer 0

#[test]
fn test_is_poisonous_single_byte_high_rank() {
    let literal = Literal::exact(vec![250]);
    let _ = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_single_byte_high_rank_boundary() {
    let literal = Literal::exact(vec![255]);
    let _ = literal.is_poisonous();
}

#[test]
fn test_is_poisonous_single_byte_rank_249() {
    let literal = Literal::exact(vec![249]);
    let _ = literal.is_poisonous();
}


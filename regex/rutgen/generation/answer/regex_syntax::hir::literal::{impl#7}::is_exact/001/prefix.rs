// Answer 0

#[test]
fn test_literal_is_exact_true() {
    let lit = Literal::exact(vec![1, 2, 3]);
    let result = lit.is_exact();
}

#[test]
fn test_literal_is_exact_false() {
    let lit = Literal::inexact(vec![1, 2, 3]);
    let result = lit.is_exact();
}


// Answer 0

#[test]
fn test_len_empty_exact_literal() {
    let lit = Literal::exact(Vec::<u8>::new());
    let length = lit.len();
}

#[test]
fn test_len_empty_inexact_literal() {
    let lit = Literal::inexact(Vec::<u8>::new());
    let length = lit.len();
}

#[test]
fn test_len_exact_literal_small() {
    let lit = Literal::exact(vec![1, 2, 3]);
    let length = lit.len();
}

#[test]
fn test_len_inexact_literal_small() {
    let lit = Literal::inexact(vec![4, 5, 6]);
    let length = lit.len();
}

#[test]
fn test_len_exact_literal_boundary() {
    let lit = Literal::exact(vec![0; 1000]);
    let length = lit.len();
}

#[test]
fn test_len_inexact_literal_boundary() {
    let lit = Literal::inexact(vec![0; 1000]);
    let length = lit.len();
}

#[test]
fn test_len_exact_literal_maximum() {
    let lit = Literal::exact(vec![255; 1000]);
    let length = lit.len();
}

#[test]
fn test_len_inexact_literal_maximum() {
    let lit = Literal::inexact(vec![255; 1000]);
    let length = lit.len();
}


// Answer 0

#[test]
fn test_extend_inexact_with_non_empty_literal() {
    let mut literal_self = Literal::inexact(vec![1, 2, 3]);
    let literal_lit = Literal::exact(vec![4, 5, 6]);
    literal_self.extend(&literal_lit);
}

#[test]
fn test_extend_inexact_with_empty_literal() {
    let mut literal_self = Literal::inexact(vec![7, 8, 9]);
    let literal_lit = Literal::exact(vec![]);
    literal_self.extend(&literal_lit);
}

#[test]
fn test_extend_inexact_with_inexact_literal() {
    let mut literal_self = Literal::inexact(vec![10, 11]);
    let literal_lit = Literal::inexact(vec![12, 13]);
    literal_self.extend(&literal_lit);
}

#[test]
fn test_extend_inexact_with_exact_literal() {
    let mut literal_self = Literal::inexact(vec![14]);
    let literal_lit = Literal::exact(vec![15, 16]);
    literal_self.extend(&literal_lit);
}

#[test]
fn test_extend_inexact_with_large_literal() {
    let mut literal_self = Literal::inexact(vec![17, 18, 19]);
    let literal_lit = Literal::exact(vec![20, 21, 22, 23, 24]);
    literal_self.extend(&literal_lit);
}


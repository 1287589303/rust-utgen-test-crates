// Answer 0

#[test]
fn test_extend_with_same_length_exact() {
    let mut lit1 = Literal::exact(vec![1, 2, 3]);
    let lit2 = Literal::exact(vec![4, 5, 6]);
    lit1.extend(&lit2);
}

#[test]
fn test_extend_with_differing_length_exact() {
    let mut lit1 = Literal::exact(vec![1, 2]);
    let lit2 = Literal::exact(vec![3, 4, 5, 6]);
    lit1.extend(&lit2);
}

#[test]
fn test_extend_with_longer_exact() {
    let mut lit1 = Literal::exact(vec![10, 20, 30]);
    let lit2 = Literal::exact(vec![40, 50, 60, 70, 80]);
    lit1.extend(&lit2);
}

#[test]
fn test_extend_with_shorter_exact() {
    let mut lit1 = Literal::exact(vec![100, 200, 300, 400]);
    let lit2 = Literal::exact(vec![500, 600]);
    lit1.extend(&lit2);
}

#[test]
fn test_extend_with_identical_exact() {
    let mut lit1 = Literal::exact(vec![7, 8, 9]);
    let lit2 = Literal::exact(vec![7, 8, 9]);
    lit1.extend(&lit2);
}


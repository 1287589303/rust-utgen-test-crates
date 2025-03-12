// Answer 0

#[test]
fn test_longest_common_prefix_non_empty_common_prefix() {
    let lit1 = Literal::exact(vec![b'f', b'o', b'o']);
    let lit2 = Literal::exact(vec![b'f', b'o', b'o', b'b', b'a', b'r']);
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_multiple_lits_same_prefix() {
    let lit1 = Literal::exact(vec![b'f', b'o', b'o']);
    let lit2 = Literal::exact(vec![b'f', b'o', b'o']);
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_single_literal() {
    let lit = Literal::exact(vec![b'f', b'o', b'o']);
    let seq = Seq::new(vec![lit]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_different_lits() {
    let lit1 = Literal::exact(vec![b'f', b'o']);
    let lit2 = Literal::exact(vec![b'b', b'a', b'r']);
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}


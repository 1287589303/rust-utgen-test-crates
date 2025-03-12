// Answer 0

#[test]
fn test_longest_common_prefix_single_literal() {
    let lit = Literal::exact(b"foo");
    let seq = Seq::new(vec![lit]);
    let _ = seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_multiple_same_literals() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"foo");
    let seq = Seq::new(vec![lit1, lit2]);
    let _ = seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_with_common_prefix() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"foobar");
    let lit3 = Literal::exact(b"fo");
    let seq = Seq::new(vec![lit1, lit2, lit3]);
    let _ = seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_with_empty_string() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"");
    let seq = Seq::new(vec![lit1, lit2]);
    let _ = seq.longest_common_prefix();
}


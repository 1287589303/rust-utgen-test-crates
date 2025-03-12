// Answer 0

#[test]
fn test_longest_common_suffix_case1() {
    let lit1 = Literal::exact(b"oof");
    let lit2 = Literal::exact(b"raboof");
    let lit3 = Literal::exact(b"of");
    let seq = Seq::new(vec![lit1, lit2, lit3]);
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_case2() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"foo");
    let seq = Seq::new(vec![lit1, lit2]);
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_case3() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"bar");
    let seq = Seq::new(vec![lit1, lit2]);
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_case4() {
    let lit = Literal::exact(b"");
    let seq = Seq::new(vec![lit]);
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_case5() {
    let lit1 = Literal::exact(b"abc");
    let lit2 = Literal::exact(b"abcde");
    let lit3 = Literal::exact(b"bc");
    let seq = Seq::new(vec![lit1, lit2, lit3]);
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_case6() {
    let lit1 = Literal::exact(b"a");
    let lit2 = Literal::exact(b"");
    let seq = Seq::new(vec![lit1, lit2]);
    let _ = seq.longest_common_suffix();
}


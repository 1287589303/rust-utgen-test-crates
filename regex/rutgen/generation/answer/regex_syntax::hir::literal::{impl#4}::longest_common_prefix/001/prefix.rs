// Answer 0

#[test]
fn test_longest_common_prefix_identical_literals() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"foo");
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_shared_prefix() {
    let lit1 = Literal::exact(b"foobar");
    let lit2 = Literal::exact(b"foobaz");
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_with_empty_string() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"");
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_different_strings() {
    let lit1 = Literal::exact(b"foo");
    let lit2 = Literal::exact(b"bar");
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_large_inputs() {
    let lit1 = Literal::exact(b"This is a long string that serves as a test case.");
    let lit2 = Literal::exact(b"This is a long string that serves as a test case, too.");
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_prefix();
}


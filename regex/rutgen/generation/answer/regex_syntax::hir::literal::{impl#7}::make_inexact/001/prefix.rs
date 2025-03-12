// Answer 0

#[test]
fn test_make_inexact_when_exact() {
    let mut literal = Literal::exact(b"test");
    literal.make_inexact();
}

#[test]
fn test_make_inexact_when_inexact() {
    let mut literal = Literal::inexact(b"test");
    literal.make_inexact();
}


// Answer 0

#[test]
fn test_minimize_no_inexact_literals() {
    let mut literals = vec![
        Literal::exact(vec![b'a']), 
        Literal::exact(vec![b'b']),
        Literal::exact(vec![b'c']),
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}

#[test]
fn test_minimize_single_exact_literal() {
    let mut literals = vec![
        Literal::exact(vec![b'x']),
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}

#[test]
fn test_minimize_multiple_literals_keep_exact_false() {
    let mut literals = vec![
        Literal::exact(vec![b'a']),
        Literal::exact(vec![b'a', b'b']),
        Literal::exact(vec![b'b', b'a']),
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}

#[test]
fn test_minimize_all_exact_liters() {
    let mut literals = vec![
        Literal::exact(vec![b'1']),
        Literal::exact(vec![b'2']),
        Literal::exact(vec![b'3']),
        Literal::exact(vec![b'4']),
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}


// Answer 0

#[test]
fn test_minimize_some_inexact() {
    let mut literals = vec![
        Literal::exact(vec![1, 2, 3]), 
        Literal::exact(vec![4, 5]), 
        Literal::inexact(vec![1, 2, 3]), 
        Literal::exact(vec![6, 7])
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}

#[test]
fn test_minimize_all_exact() {
    let mut literals = vec![
        Literal::exact(vec![1, 2, 3]), 
        Literal::exact(vec![4, 5]), 
        Literal::exact(vec![6, 7])
    ];
    let keep_exact = true;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}

#[test]
fn test_minimize_mixed_exact_and_inexact() {
    let mut literals = vec![
        Literal::exact(vec![1, 2, 3]), 
        Literal::inexact(vec![1, 2, 3]), 
        Literal::exact(vec![4, 5]), 
        Literal::inexact(vec![4, 5])
    ];
    let keep_exact = false;
    PreferenceTrie::minimize(&mut literals, keep_exact);
}


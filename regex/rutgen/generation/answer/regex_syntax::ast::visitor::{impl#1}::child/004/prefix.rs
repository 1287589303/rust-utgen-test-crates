// Answer 0

#[test]
fn test_child_with_repetition_min_zero() {
    let repetition = Repetition {
        min: 0,
        max: Some(5),
        greedy: true,
        sub: Box::new(Ast::Literal(Box::new(Literal::new('a')))),
    };
    let frame = Frame::Repetition(&repetition);
    let _result = frame.child();
}

#[test]
fn test_child_with_repetition_min_one() {
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: false,
        sub: Box::new(Ast::Literal(Box::new(Literal::new('b')))),
    };
    let frame = Frame::Repetition(&repetition);
    let _result = frame.child();
}

#[test]
fn test_child_with_repetition_max_none() {
    let repetition = Repetition {
        min: 2,
        max: None,
        greedy: true,
        sub: Box::new(Ast::Literal(Box::new(Literal::new('c')))),
    };
    let frame = Frame::Repetition(&repetition);
    let _result = frame.child();
}

#[test]
fn test_child_with_repetition_greedy_false() {
    let repetition = Repetition {
        min: 0,
        max: Some(10),
        greedy: false,
        sub: Box::new(Ast::Concat(Box::new(Concat::new(vec![
            Ast::Literal(Box::new(Literal::new('d'))),
            Ast::Literal(Box::new(Literal::new('e'))),
        ])))),
    };
    let frame = Frame::Repetition(&repetition);
    let _result = frame.child();
}


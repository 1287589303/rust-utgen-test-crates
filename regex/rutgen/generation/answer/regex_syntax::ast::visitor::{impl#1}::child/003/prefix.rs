// Answer 0

#[test]
fn test_child_group_with_literal() {
    let literal = ast::Ast::Literal(Box::new(Literal::new('a')));
    let group = ast::Group {
        span: Span::new(0, 1),
        kind: GroupKind::Capturing,
        ast: Box::new(literal),
    };
    let frame = Frame::Group(&group);
    let _child = frame.child();
}

#[test]
fn test_child_group_with_concatenation() {
    let literals = vec![
        ast::Ast::Literal(Box::new(Literal::new('a'))),
        ast::Ast::Literal(Box::new(Literal::new('b'))),
    ];
    let concat = ast::Ast::Concat(Box::new(Concat::new(literals)));
    
    let group = ast::Group {
        span: Span::new(0, 2),
        kind: GroupKind::Capturing,
        ast: Box::new(concat),
    };
    let frame = Frame::Group(&group);
    let _child = frame.child();
}

#[test]
fn test_child_group_with_alt() {
    let alt = ast::Ast::Alternation(Box::new(Alternation::new(vec![
        ast::Ast::Literal(Box::new(Literal::new('a'))),
        ast::Ast::Literal(Box::new(Literal::new('b'))),
    ])));
    
    let group = ast::Group {
        span: Span::new(0, 2),
        kind: GroupKind::Capturing,
        ast: Box::new(alt),
    };
    let frame = Frame::Group(&group);
    let _child = frame.child();
}

#[test]
fn test_child_group_with_repetition() {
    let repetition = ast::Ast::Repetition(Box::new(ast::Repetition {
        min: 1,
        max: Some(2),
        greedy: true,
        sub: Box::new(ast::Ast::Literal(Box::new(Literal::new('a')))),
    }));
    
    let group = ast::Group {
        span: Span::new(0, 3),
        kind: GroupKind::Capturing,
        ast: Box::new(repetition),
    };
    let frame = Frame::Group(&group);
    let _child = frame.child();
}


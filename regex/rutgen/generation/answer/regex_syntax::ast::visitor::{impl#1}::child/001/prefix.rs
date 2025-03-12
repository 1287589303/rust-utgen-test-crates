// Answer 0

#[test]
fn test_child_alternation_non_empty_head_literal() {
    let literal = Ast::Literal(Box::new(Literal::new('a')));
    let frame = Frame::Alternation {
        head: &literal,
        tail: &[],
    };
    let _ = frame.child();
}

#[test]
fn test_child_alternation_non_empty_head_group() {
    let group_ast = Ast::Group(Box::new(Group {
        span: Span::default(),
        kind: GroupKind::Capture(1),
        ast: Box::new(Ast::Literal(Box::new(Literal::new('b')))),
    }));
    let frame = Frame::Alternation {
        head: &group_ast,
        tail: &[],
    };
    let _ = frame.child();
}

#[test]
fn test_child_alternation_non_empty_head_repetition() {
    let repetition_ast = Ast::Repetition(Box::new(Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Ast::Literal(Box::new(Literal::new('c')))),
    }));
    let frame = Frame::Alternation {
        head: &repetition_ast,
        tail: &[],
    };
    let _ = frame.child();
}

#[test]
fn test_child_alternation_non_empty_head_concat() {
    let concat_ast = Ast::Concat(Box::new(Concat {
        nodes: vec![
            Ast::Literal(Box::new(Literal::new('d'))),
            Ast::Literal(Box::new(Literal::new('e'))),
        ],
    }));
    let frame = Frame::Alternation {
        head: &concat_ast,
        tail: &[],
    };
    let _ = frame.child();
}


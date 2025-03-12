// Answer 0

#[test]
fn test_repetition_with_min_zero_max_none() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}

#[test]
fn test_repetition_with_min_zero_max_zero() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 0,
        max: Some(0),
        greedy: true,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}

#[test]
fn test_repetition_with_min_one_max_ten() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 1,
        max: Some(10),
        greedy: false,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}

#[test]
fn test_repetition_with_min_ten_max_ten() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 10,
        max: Some(10),
        greedy: true,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}

#[test]
fn test_repetition_with_min_three_max_five() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 3,
        max: Some(5),
        greedy: false,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}

#[test]
fn test_repetition_with_min_five_max_none() {
    let span = Span { start: Position(0), end: Position(1) };
    let sub_ast = Ast::empty(span.clone());
    let repetition = Repetition {
        min: 5,
        max: None,
        greedy: true,
        sub: Box::new(sub_ast),
    };
    let result = Ast::repetition(repetition);
}


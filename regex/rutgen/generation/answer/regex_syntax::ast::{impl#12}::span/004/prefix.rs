// Answer 0

#[test]
fn test_span_repetition_min_zero_max_none() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expr = Box::new(Ast::Literal(Box::new(Literal(Vec::from("a".as_bytes()), true))));
    let repetition = Repetition { min: 0, max: None, greedy: true, sub: sub_expr };
    let ast = Ast::Repetition(Box::new(repetition));
    let _ = ast.span();
}

#[test]
fn test_span_repetition_min_zero_max_ten() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expr = Box::new(Ast::Literal(Box::new(Literal(Vec::from("b".as_bytes()), false))));
    let repetition = Repetition { min: 0, max: Some(10), greedy: false, sub: sub_expr };
    let ast = Ast::Repetition(Box::new(repetition));
    let _ = ast.span();
}

#[test]
fn test_span_repetition_min_ten_max_ten() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expr = Box::new(Ast::Literal(Box::new(Literal(Vec::from("c".as_bytes()), true))));
    let repetition = Repetition { min: 10, max: Some(10), greedy: true, sub: sub_expr };
    let ast = Ast::Repetition(Box::new(repetition));
    let _ = ast.span();
}

#[test]
fn test_span_repetition_min_five_max_none() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expr = Box::new(Ast::Literal(Box::new(Literal(Vec::from("d".as_bytes()), false))));
    let repetition = Repetition { min: 5, max: None, greedy: false, sub: sub_expr };
    let ast = Ast::Repetition(Box::new(repetition));
    let _ = ast.span();
}


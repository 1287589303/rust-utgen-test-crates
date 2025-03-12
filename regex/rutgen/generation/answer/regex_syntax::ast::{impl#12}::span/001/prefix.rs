// Answer 0

#[test]
fn test_span_concat_valid() {
    let start_position = Position(0);
    let end_position = Position(5);
    
    let span = Span { start: start_position, end: end_position };
    let ast1 = Ast::literal(Literal(Box::new([b'a', b'b', b'c'].into_boxed_slice())));
    let ast2 = Ast::literal(Literal(Box::new([b'd', b'e', b'f'].into_boxed_slice())));
    
    let concat = Ast::concat(Concat { span, asts: vec![ast1, ast2] });
    
    let result = concat.span();
}

#[test]
fn test_span_concat_empty() {
    let start_position = Position(0);
    let end_position = Position(0);
    
    let span = Span { start: start_position, end: end_position };
    
    let concat = Ast::concat(Concat { span, asts: vec![] });
    
    let result = concat.span();
}

#[test]
fn test_span_concat_single_element() {
    let start_position = Position(2);
    let end_position = Position(3);
    
    let span = Span { start: start_position, end: end_position };
    let ast = Ast::literal(Literal(Box::new([b'x'].into_boxed_slice())));
    
    let concat = Ast::concat(Concat { span, asts: vec![ast] });
    
    let result = concat.span();
}

#[test]
fn test_span_concat_boundary_conditions() {
    let start_position = Position(0);
    let end_position = Position(1);
    
    let span = Span { start: start_position, end: end_position };
    let ast = Ast::literal(Literal(Box::new([b'y'].into_boxed_slice())));
    
    let concat = Ast::concat(Concat { span, asts: vec![ast] });
    
    let result = concat.span();
}

#[test]
fn test_span_concat_large_offsets() {
    let start_position = Position(100);
    let end_position = Position(200);
    
    let span = Span { start: start_position, end: end_position };
    let ast1 = Ast::literal(Literal(Box::new([b'z'].into_boxed_slice())));
    let ast2 = Ast::dot(span);
    
    let concat = Ast::concat(Concat { span, asts: vec![ast1, ast2] });
    
    let result = concat.span();
}


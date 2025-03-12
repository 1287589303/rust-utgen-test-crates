// Answer 0

#[test]
fn test_push_group_with_flags() {
    let span = Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 10 } };
    let flags_item = FlagsItem { kind: FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) };
    let flags = Flags { span: span.clone(), items: vec![flags_item] };
    
    let group = Group { span: span.clone(), kind: GroupKind::NonCapturing(flags.clone()), ast: Box::new(Ast::empty(span.clone())) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::group(group.clone())] };
    
    let parser = Parser { /*initialize necessary fields*/ };

    let parser_i = ParserI::new(&parser, "(?:" );
    parser_i.push_group(concat).unwrap();
}

#[test]
fn test_push_group_without_flags() {
    let span = Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 10 } };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing, ast: Box::new(Ast::empty(span.clone())) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::group(group.clone())] };

    let parser = Parser { /*initialize necessary fields*/ };

    let parser_i = ParserI::new(&parser, "(abc" );
    parser_i.push_group(concat).unwrap();
}

#[test]
fn test_push_group_empty_concat() {
    let span = Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 1 } };
    let concat = Concat { span: span.clone(), asts: vec![] };

    let parser = Parser { /*initialize necessary fields*/ };

    let parser_i = ParserI::new(&parser, "(" );
    parser_i.push_group(concat).unwrap();
}

#[test]
#[should_panic]
fn test_push_group_no_open_parenthesis() {
    let span = Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 10 } };
    let concat = Concat { span: span.clone(), asts: vec![] };

    let parser = Parser { /*initialize necessary fields*/ };
    
    let parser_i = ParserI::new(&parser, "abc" );
    parser_i.push_group(concat);
}


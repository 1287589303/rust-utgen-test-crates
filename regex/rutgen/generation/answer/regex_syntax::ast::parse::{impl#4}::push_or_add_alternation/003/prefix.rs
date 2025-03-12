// Answer 0

#[test]
fn test_push_or_add_alternation_existing_alternation() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let concat_span = Span::new(start_position, end_position);
    let concat_ast = Ast::Concat(Box::new(Concat {
        span: concat_span.clone(),
        asts: vec![],
    }));

    let mut stack_group = RefCell::new(vec![
        GroupState::Alternation(ast::Alternation {
            span: concat_span.clone(),
            asts: vec![concat_ast.clone()],
        }),
    ]);
    
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group,
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, ".*");
    
    let new_concat = Ast::Concat(Box::new(Concat {
        span: concat_span,
        asts: vec![],
    }));

    parser_instance.push_or_add_alternation(new_concat);
}

#[test]
fn test_push_or_add_alternation_multiple_concats() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let concat_span = Span::new(start_position, end_position);
    let concat_ast = Ast::Concat(Box::new(Concat {
        span: concat_span.clone(),
        asts: vec![Ast::Literal(Box::new(ast::Literal { value: 'a' }))]
    }));

    let mut stack_group = RefCell::new(vec![
        GroupState::Alternation(ast::Alternation {
            span: concat_span.clone(),
            asts: vec![concat_ast.clone()],
        }),
    ]);
    
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group,
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, ".*");
    
    let new_concat = Ast::Concat(Box::new(Concat {
        span: concat_span,
        asts: vec![Ast::Literal(Box::new(ast::Literal { value: 'b' }))]
    }));

    parser_instance.push_or_add_alternation(new_concat);
}


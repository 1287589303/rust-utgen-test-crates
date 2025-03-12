// Answer 0

#[test]
fn test_visit_class_set_binary_op_pre_with_minimal_depth() {
    let span = Span { start: 0, end: 0 };
    let ast = ast::ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::default()), rhs: Box::new(ClassSet::default()) };
    
    let parser = Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.depth = 0;

    limiter.visit_class_set_binary_op_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_binary_op_pre_with_exact_depth_limit() {
    let span = Span { start: 0, end: 0 };
    let ast = ast::ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::default()), rhs: Box::new(ClassSet::default()) };
    
    let parser = Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.depth = 1;

    limiter.visit_class_set_binary_op_pre(&ast).unwrap();
}

#[test]
#[should_panic]
fn test_visit_class_set_binary_op_pre_exceeding_depth_limit() {
    let span = Span { start: 0, end: 0 };
    let ast = ast::ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::default()), rhs: Box::new(ClassSet::default()) };

    let parser = Parser { 
        pos: Cell::new(Position::default()), 
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI { parser, pattern: "" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.depth = 2;

    limiter.visit_class_set_binary_op_pre(&ast).unwrap();
}


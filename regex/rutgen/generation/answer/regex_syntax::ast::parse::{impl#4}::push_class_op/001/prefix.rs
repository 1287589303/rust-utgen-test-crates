// Answer 0

#[test]
fn test_push_class_op_intersection() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
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
    let parser_instance = ParserI::new(&parser, "some_pattern");
    
    let next_kind = ast::ClassSetBinaryOpKind::Intersection;
    let next_union = ClassSetUnion {
        span: Span { start: 0, end: 1 },
        items: vec![ClassSetItem::Literal(Literal::new('a'))],
    };
    
    parser_instance.push_class_op(next_kind, next_union);
}

#[test]
fn test_push_class_op_difference() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
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
    let parser_instance = ParserI::new(&parser, "some_pattern");

    let next_kind = ast::ClassSetBinaryOpKind::Difference;
    let next_union = ClassSetUnion {
        span: Span { start: 1, end: 2 },
        items: vec![ClassSetItem::Literal(Literal::new('b'))],
    };

    parser_instance.push_class_op(next_kind, next_union);
}

#[test]
fn test_push_class_op_symmetric_difference() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
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
    let parser_instance = ParserI::new(&parser, "some_pattern");

    let next_kind = ast::ClassSetBinaryOpKind::SymmetricDifference;
    let next_union = ClassSetUnion {
        span: Span { start: 2, end: 3 },
        items: vec![ClassSetItem::Literal(Literal::new('c'))],
    };

    parser_instance.push_class_op(next_kind, next_union);
}

#[test]
fn test_push_class_op_with_non_empty_stack() {
    let parser = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![
            ClassState::Open {
                union: ClassSetUnion {
                    span: Span { start: 0, end: 1 },
                    items: vec![ClassSetItem::Literal(Literal::new('x'))],
                },
                set: ClassBracketed::new(),
            },
        ]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_instance = ParserI::new(&parser, "some_pattern");

    let next_kind = ast::ClassSetBinaryOpKind::Intersection;
    let next_union = ClassSetUnion {
        span: Span { start: 1, end: 2 },
        items: vec![ClassSetItem::Literal(Literal::new('y'))],
    };

    parser_instance.push_class_op(next_kind, next_union);
}


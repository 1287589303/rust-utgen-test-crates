// Answer 0

#[test]
fn test_pop_class_with_non_empty_stack() {
    let span = Span::new(0, 5);
    let nested_union = ClassSetUnion {
        span: span.clone(),
        items: vec![ClassSetItem::Literal(Literal::from('a'))],
    };

    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: span.clone(),
                items: vec![],
            },
            set: ClassBracketed {
                span: span.clone(),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('b'))),
            },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, "[a]");
    parser_i.pop_class(nested_union);
}

#[test]
fn test_pop_class_with_single_class_state() {
    let span = Span::new(0, 3);
    let nested_union = ClassSetUnion {
        span: span.clone(),
        items: vec![ClassSetItem::Literal(Literal::from('c'))],
    };

    let parser = Parser {
        pos: Cell::new(Position { offset: 2, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: Span::new(0, 0),
                items: vec![],
            },
            set: ClassBracketed {
                span: span.clone(),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('d'))),
            },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, "[c]");
    parser_i.pop_class(nested_union);
}

#[test]
fn test_pop_class_with_nil_open_union() {
    let span = Span::new(0, 4);
    let nested_union = ClassSetUnion {
        span: span.clone(),
        items: vec![ClassSetItem::Literal(Literal::from('e'))],
    };

    let parser = Parser {
        pos: Cell::new(Position { offset: 3, line: 1, column: 3 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: Span::new(0, 2),
                items: vec![ClassSetItem::Literal(Literal::from('f'))],
            },
            set: ClassBracketed {
                span: span.clone(),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('g'))),
            },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, "[e]");
    parser_i.pop_class(nested_union);
}


// Answer 0

#[test]
fn test_pop_class_success_with_non_empty_stack() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion { span: Span::new(0, 5), items: vec![] },
            set: ClassBracketed { span: Span::new(0, 5), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))) },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "a]");
    parser_i.bump(); // move to character at position 2, which must be ']'
    let nested_union = ClassSetUnion { span: Span::new(0, 5), items: vec![ClassSetItem::Literal(Literal::from('a'))] };
    let _result = parser_i.pop_class(nested_union);
}

#[test]
#[should_panic]
fn test_pop_class_panic_unexpected_empty_stack() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]), // empty stack
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "]"); // position at ']'
    let nested_union = ClassSetUnion { span: Span::new(0, 1), items: vec![] }; // empty nested union
    let _result = parser_i.pop_class(nested_union);
}

#[test]
fn test_pop_class_success_with_open_state() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion { span: Span::new(0, 6), items: vec![] },
            set: ClassBracketed { span: Span::new(0, 6), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('b'))) },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "b]");
    parser_i.bump(); // position at ']'
    let nested_union = ClassSetUnion { span: Span::new(0, 6), items: vec![ClassSetItem::Literal(Literal::from('b'))] };
    let _result = parser_i.pop_class(nested_union);
}


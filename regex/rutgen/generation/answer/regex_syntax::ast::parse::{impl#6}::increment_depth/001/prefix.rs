// Answer 0

#[test]
fn test_increment_depth_exceeds_max_depth() {
    let span = Span { start: 0, end: 1 };
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: u32::MAX,
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

    let parser_i = ParserI::new(Box::new(parser), "test pattern");
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.depth = u32::MAX - 1;
    
    let _ = limiter.increment_depth(&span);
}

#[test]
fn test_increment_depth_exceeds_nest_limit() {
    let span = Span { start: 0, end: 1 };
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: u32::MAX - 1,
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

    let parser_i = ParserI::new(Box::new(parser), "test pattern");
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.depth = u32::MAX; // Setting depth to max to exceed the set limit
    
    let _ = limiter.increment_depth(&span);
}


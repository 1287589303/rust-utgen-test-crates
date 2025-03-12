// Answer 0

#[test]
fn test_increment_depth_ok() {
    let pattern = "a*b+c?";
    let span = Span { start: 0, end: 1 };
    let limit = 5; // choose a limit greater than depth
    let depth = 5; // set depth to the limit
    
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: limit,
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

    let parser_i = ParserI::new(&parser, pattern);
    let nest_limiter = NestLimiter {
        p: &parser_i,
        depth,
    };

    let result = nest_limiter.increment_depth(&span);
}


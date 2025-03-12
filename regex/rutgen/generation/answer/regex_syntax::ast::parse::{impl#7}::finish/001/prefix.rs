// Answer 0

#[test]
fn test_finish_valid_depth_zero() {
    let pattern = "a*b";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 3,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter {
        p: &parser,
        depth: 0,
    };
    
    nest_limiter.finish().unwrap();
}

#[test]
fn test_finish_valid_depth_non_zero() {
    let pattern = "a(b|c)*";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 3,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter {
        p: &parser,
        depth: 1,
    };

    nest_limiter.finish().unwrap();
}

#[test]
fn test_finish_valid_depth_at_nest_limit() {
    let pattern = "((a|b)c|d)";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 2,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let nest_limiter = NestLimiter {
        p: &parser,
        depth: 2,
    };

    nest_limiter.finish().unwrap();
}


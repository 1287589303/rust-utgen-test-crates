// Answer 0

#[test]
fn test_bump_if_false_empty_pattern() {
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
        capture_index: Cell::new(0), 
        nest_limit: 0, 
        octal: false, 
        initial_ignore_whitespace: false, 
        empty_min_range: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new()) 
    }, "");
    assert_eq!(parser.bump_if("abc"), false);
}

#[test]
fn test_bump_if_false_no_matching_prefix() {
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
        capture_index: Cell::new(0), 
        nest_limit: 0, 
        octal: false, 
        initial_ignore_whitespace: false, 
        empty_min_range: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::from("xyz")) 
    }, "xyz");
    assert_eq!(parser.bump_if("abc"), false);
}

#[test]
fn test_bump_if_false_prefix_longer_than_pattern() {
    let parser = ParserI::new(Parser { 
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
        capture_index: Cell::new(0), 
        nest_limit: 0, 
        octal: false, 
        initial_ignore_whitespace: false, 
        empty_min_range: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::from("ab")) 
    }, "ab");
    assert_eq!(parser.bump_if("abc"), false);
}


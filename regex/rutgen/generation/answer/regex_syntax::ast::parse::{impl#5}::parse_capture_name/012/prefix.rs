// Answer 0

#[test]
fn test_parse_capture_name_eof_before_closing() {
    let pattern = "<valid_name>";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
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
        },
        pattern: pattern,
    };
    parser.parser.pos.set(Position { offset: 1, line: 1, column: 2 }); // Positioned after the '<'
    
    // Simulate character '>' and return that `bump` is not invoked.
    // This will trigger the pending error due to EOF.
    assert!(parser.parse_capture_name(0).is_err());
} 

#[test]
fn test_parse_capture_name_invalid_character() {
    let pattern = "<invalid_name>"; 
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
            capture_index: Cell::new(1),
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
        },
        pattern: pattern,
    };
    parser.parser.pos.set(Position { offset: 1, line: 1, column: 2 }); // Positioned after the '<'

    // Simulate character '>' while invalid character present
    let invalid_char_position = 4; // Simulated position of invalid character
    parser.parser.pos.set(Position { offset: invalid_char_position, line: 1, column: invalid_char_position + 1 });
    // Simulate the expected return value in case of invalid name character
    assert!(parser.parse_capture_name(1).is_err());
}

#[test]
fn test_parse_capture_name_empty_name() {
    let pattern = "<>"; 
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
            capture_index: Cell::new(2),
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
        },
        pattern: pattern,
    };
    parser.parser.pos.set(Position { offset: 1, line: 1, column: 2 }); // Positioned after the '<'

    // Affecting the parser's state so that the closing '>' is taken into account
    parser.parser.pos.set(Position { offset: 2, line: 1, column: 3 }); // Before the '>'
    
    // Set EOF state for the parser (post-first bump) 
    parser.parser.pos.set(Position { offset: 3, line: 1, column: 4 }); 
    assert!(parser.parse_capture_name(2).is_err());
}


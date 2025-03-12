// Answer 0

#[test]
fn test_span_char_non_newline() {
    let position = Position { offset: 4, line: 1, column: 5 };
    let parser = Parser {
        pos: Cell::new(position),
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
  
    let pattern = "abcde";
    let parser_i = ParserI::new(&parser, pattern);

    // Setting a non-newline character at the current position
    let _ = parser_i.char_at(4); // Should return 'e' for this input
    let span = parser_i.span_char(); // Call the function under test
}

#[test]
fn test_span_char_valid_offset() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
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
  
    let pattern = "XYZ";
    let parser_i = ParserI::new(&parser, pattern);

    // Ensure that we're not at the end of the string
    let _ = parser_i.char_at(0); // Should return 'X'
    let span = parser_i.span_char(); // Call the function under test
}

#[test]
fn test_span_char_within_bounds() {
    let position = Position { offset: 2, line: 1, column: 3 };
    let parser = Parser {
        pos: Cell::new(position),
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
  
    let pattern = "hello";
    let parser_i = ParserI::new(&parser, pattern);

    // Confirming that the char_at method provides valid non-newline character
    let _ = parser_i.char_at(2); // Should return 'l'
    let span = parser_i.span_char(); // Call the function under test
}

#[test]
fn test_span_char_offset_boundary() {
    let position = Position { offset: 4, line: 1, column: 5 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
  
    let pattern = "abcd"; // Offset at 4 must be at length - 1
    let parser_i = ParserI::new(&parser, pattern);

    // Triggering chars at max offset
    let _ = parser_i.char_at(3); // Should return 'd'
    let span = parser_i.span_char(); // Call the function under test
}


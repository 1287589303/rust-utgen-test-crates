// Answer 0

#[test]
fn test_span_start_position() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) };
    let parser_i = ParserI::new(&parser, ".*");
    let _span = parser_i.span();
}

#[test]
fn test_span_middle_position() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let parser = Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) };
    let parser_i = ParserI::new(&parser, ".*");
    let _span = parser_i.span();
}

#[test]
fn test_span_end_position() {
    let position = Position { offset: 10, line: 1, column: 11 };
    let parser = Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) };
    let parser_i = ParserI::new(&parser, ".*");
    let _span = parser_i.span();
}

#[test]
fn test_span_empty_pattern() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) };
    let parser_i = ParserI::new(&parser, "");
    let _span = parser_i.span();
}


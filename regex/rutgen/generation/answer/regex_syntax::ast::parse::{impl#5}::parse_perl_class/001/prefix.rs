// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "d",
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_negated_digit() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "D",
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_space() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "s",
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_negated_space() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "S",
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_word() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "w",
    };
    parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_negated_word() {
    let parser = ParserI {
        parser: Parser { pos: Cell::new(Position(0)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) },
        pattern: "W",
    };
    parser.parse_perl_class();
}


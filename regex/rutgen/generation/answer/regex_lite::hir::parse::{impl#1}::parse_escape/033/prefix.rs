// Answer 0

#[test]
fn test_parse_escape_with_character_w() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\w";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('w')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_d() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\d";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('d')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_D() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\D";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('D')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_u() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\u";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('u')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_x() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\x";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('x')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_U() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\U";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('U')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_s() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\s";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('s')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_P() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\P";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('P')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_p() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\p";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('p')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_character_S() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "\\S";
    let parser = Parser { config, pattern, depth: Cell::new(1), pos: Cell::new(0), char: Cell::new(Some('S')), capture_index: Cell::new(0), flags: RefCell::new(Flags::default()), capture_names: RefCell::new(vec![]) };
    let result = parser.parse_escape();
}


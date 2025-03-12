// Answer 0

#[test]
fn test_parse_class_item_valid_character_a() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "a",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}

#[test]
fn test_parse_class_item_valid_character_b() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "b",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('b')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}

#[test]
fn test_parse_class_item_valid_character_z() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "z",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('z')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}

#[test]
fn test_parse_class_item_valid_character_1() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "1",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('1')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}

#[test]
fn test_parse_class_item_valid_character_space() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: " ",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(' ')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}

#[test]
fn test_parse_class_item_valid_character_special() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "!@#",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('!')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_item();
}


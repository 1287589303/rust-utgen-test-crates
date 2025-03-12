// Answer 0

#[test]
fn test_parse_escape_with_w() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "w",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('w')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_p() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "p",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('p')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_d() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "d",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_D() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "D",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('D')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_u() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "u",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('u')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_x() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "x",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('x')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_U() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "U",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('U')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_s() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "s",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('s')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_P() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "P",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('P')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_S() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "S",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('S')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_n() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "n",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('n')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_r() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "r",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('r')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_f() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "f",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('f')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_t() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "t",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('t')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_v() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "v",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('v')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_b() {
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
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_a() {
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
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_B() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "B",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('B')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_angle_bracket_open() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "<",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('<')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_angle_bracket_close() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: ">",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('>')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_A() {
    let parser = Parser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "A",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('A')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_z() {
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
    let _ = parser.parse_escape();
}


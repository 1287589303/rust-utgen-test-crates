// Answer 0

#[test]
fn test_parse_escape_d() {
    let pattern = "\\d";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_D() {
    let pattern = "\\D";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_s() {
    let pattern = "\\s";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_S() {
    let pattern = "\\S";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_w() {
    let pattern = "\\w";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_W() {
    let pattern = "\\W";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_p() {
    let pattern = "\\p{L}";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_P() {
    let pattern = "\\P{L}";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_x() {
    let pattern = "\\x7F";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_x_brace() {
    let pattern = "\\x{7F}";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_U() {
    let pattern = "\\U0000007F";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_u() {
    let pattern = "\\u007F";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_n() {
    let pattern = "\\n";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_t() {
    let pattern = "\\t";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_r() {
    let pattern = "\\r";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_f() {
    let pattern = "\\f";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_v() {
    let pattern = "\\v";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_a() {
    let pattern = "\\a";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_angle_bracket_start() {
    let pattern = "<";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_angle_bracket_end() {
    let pattern = ">";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_b() {
    let pattern = "\\b";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}

#[test]
fn test_parse_escape_B() {
    let pattern = "\\B";
    let parser = ParserI {
        parser: Parser { octal: false },
        pattern,
    };
    parser.parse_escape().unwrap();
}


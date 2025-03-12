// Answer 0

#[test]
fn test_parse_escape_with_zero_to_seven() {
    let parser = ParserI {
        parser: Parser { octal: false, .. },
        pattern: "\\d",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_eight_nine() {
    let parser = ParserI {
        parser: Parser { octal: false, .. },
        pattern: "\\8",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_d() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\D",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_w() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\W",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_P() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\P{scx=Katakana}",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_lowercase_p() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\p{scx=Katakana}",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_lowercase_w() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\w",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_lowercase_s() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\s",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_s() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\S",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_lowercase_d() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\d",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_uppercase_U() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\U0001F600",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_lowercase_u() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\u1234",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_x() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\x61",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_v() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\v",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_b() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\b{start}",
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_with_non_meta_character() {
    let parser = ParserI {
        parser: Parser { octal: true, .. },
        pattern: "\\a",
    };
    parser.parse_escape();
}


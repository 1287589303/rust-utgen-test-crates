// Answer 0

#[test]
fn test_parse_hex_x_valid() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\xFF",
    };
    // Assuming the cursor is pointed at 'x'
    parser.char = 'x';
    parser.bump_and_bump_space();
    parser.char = 'F';
    parser.parse_hex();
}

#[test]
fn test_parse_hex_x_invalid() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\xG",
    };
    parser.char = 'x';
    parser.bump_and_bump_space();
    parser.char = 'G'; // Invalid hexadecimal
    parser.parse_hex();
}

#[test]
fn test_parse_hex_u_valid() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\u{FFFF}",
    };
    parser.char = 'u';
    parser.bump_and_bump_space();
    parser.char = '{';
    parser.bump_and_bump_space();
    parser.char = 'F';
    parser.parse_hex();
}

#[test]
fn test_parse_hex_u_empty() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\u{}",
    };
    parser.char = 'u';
    parser.bump_and_bump_space();
    parser.char = '{';
    parser.bump_and_bump_space();
    parser.char = '}'; // Empty hex literal
    parser.parse_hex();
}

#[test]
fn test_parse_hex_U_valid() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\U0001FFFF",
    };
    parser.char = 'U';
    parser.bump_and_bump_space();
    parser.char = '0';
    parser.parse_hex();
}

#[test]
fn test_parse_hex_U_invalid_character() {
    let parser = ParserI {
        parser: Parser { /* initialize members */ },
        pattern: r"\U00G",
    };
    parser.char = 'U';
    parser.bump_and_bump_space();
    parser.char = 'G'; // Invalid hexadecimal digit
    parser.parse_hex();
}


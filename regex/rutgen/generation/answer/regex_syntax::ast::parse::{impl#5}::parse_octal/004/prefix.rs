// Answer 0

#[test]
fn test_parse_octal_valid_0() {
    let pattern = "0";
    let parser = Parser { octal: true, ..Default::default() };
    let parser_i = ParserI { parser: Box::new(parser), pattern };

    let result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_7() {
    let pattern = "7";
    let parser = Parser { octal: true, ..Default::default() };
    let parser_i = ParserI { parser: Box::new(parser), pattern };

    let result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_77() {
    let pattern = "77";
    let parser = Parser { octal: true, ..Default::default() };
    let parser_i = ParserI { parser: Box::new(parser), pattern };

    let result = parser_i.parse_octal();
}

#[test]
fn test_parse_octal_valid_777() {
    let pattern = "777";
    let parser = Parser { octal: true, ..Default::default() };
    let parser_i = ParserI { parser: Box::new(parser), pattern };

    let result = parser_i.parse_octal();
}


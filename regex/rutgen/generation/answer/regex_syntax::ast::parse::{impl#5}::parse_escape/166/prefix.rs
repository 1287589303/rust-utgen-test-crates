// Answer 0

#[test]
fn test_parse_escape_digit() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_not_digit() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_digit() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_perl_class_whitespace() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_unicode_class() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);
    
    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_hex() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_superfluous() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let pattern = "\\d\\D\\w\\W\\s\\S\\p\\P\\x\\u\\U\\a";
    
    let parser = Parser {
        ast: ast::parse::Parser {},
        hir: hir::translate::Translator {},
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: pattern,
    };

    parser_i.pos.set(start_pos);

    let result = parser_i.parse_escape();
}


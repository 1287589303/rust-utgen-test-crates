// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    #[derive(Borrow)]
    struct MockParser {
        ignore_whitespace: Cell<bool>,
    }

    let parser = MockParser {
        ignore_whitespace: Cell::new(true),
    };

    let parser_i = ParserI::new(&parser, "some pattern");
    let result = parser_i.ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_false() {
    #[derive(Borrow)]
    struct MockParser {
        ignore_whitespace: Cell<bool>,
    }

    let parser = MockParser {
        ignore_whitespace: Cell::new(false),
    };

    let parser_i = ParserI::new(&parser, "another pattern");
    let result = parser_i.ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_alternate_states() {
    #[derive(Borrow)]
    struct MockParser {
        ignore_whitespace: Cell<bool>,
    }

    let parser_true = MockParser {
        ignore_whitespace: Cell::new(true),
    };
    
    let parser_false = MockParser {
        ignore_whitespace: Cell::new(false),
    };

    let parser_i_true = ParserI::new(&parser_true, "pattern with ignore");
    let result_true = parser_i_true.ignore_whitespace();

    let parser_i_false = ParserI::new(&parser_false, "pattern without ignore");
    let result_false = parser_i_false.ignore_whitespace();
}


// Answer 0

#[test]
fn test_parse_set_class_range_valid_prim1_invalid_prim2() {
    let position1 = Position { offset: 0, line: 1, column: 1 };
    let position2 = Position { offset: 1, line: 1, column: 2 };
    
    let lit1 = Literal {
        span: Span::new(position1, position1),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };

    let lit2 = Literal {
        span: Span::new(position2, position2),
        kind: ast::LiteralKind::Verbatim,
        c: 'b',
    };

    let valid_primitive1 = Primitive::Literal(lit1);
    let invalid_primitive2 = Primitive::Literal(lit2);
    
    let parser = Parser {
        pos: Cell::new(position1),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a-b]",
    };

    // Mock methods to simulate behavior
    impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
        fn parse_set_class_item(&self) -> Result<Primitive> {
            Ok(valid_primitive1)
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn char(&self) -> char {
            '-'
        }

        fn peek_space(&self) -> Option<char> {
            Some('c') // not ']' or '-'
        }

        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn into_class_literal(&self, _p: &Primitive) -> Result<ast::Literal> {
            Ok(lit1)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::ClassRangeInvalid,
                pattern: self.pattern.to_string(),
                span: Span::new(position1, position1),
            }
        }
    }

    let result = parser_i.parse_set_class_range();
} 

#[test]
fn test_parse_set_class_range_valid_prim1_invalid_prim2_different() {
    let position1 = Position { offset: 0, line: 1, column: 1 };
    
    let lit1 = Literal {
        span: Span::new(position1, position1),
        kind: ast::LiteralKind::Verbatim,
        c: 'x',
    };

    let lit2 = Literal {
        span: Span::new(position1, position1),
        kind: ast::LiteralKind::Verbatim,
        c: 'y',
    };

    let valid_primitive1 = Primitive::Literal(lit1);
    let invalid_primitive2 = Primitive::Literal(lit2);
    
    let parser = Parser {
        pos: Cell::new(position1),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[x-y]",
    };

    // Mock methods to simulate behavior
    impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
        fn parse_set_class_item(&self) -> Result<Primitive> {
            Ok(valid_primitive1)
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn char(&self) -> char {
            '-'
        }

        fn peek_space(&self) -> Option<char> {
            Some('c') // not ']' or '-'
        }

        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn into_class_literal(&self, _p: &Primitive) -> Result<ast::Literal> {
            Err(ast::Error {
                kind: ast::ErrorKind::ClassRangeInvalid,
                pattern: self.pattern.to_string(),
                span: Span::new(position1, position1),
            })
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::ClassRangeInvalid,
                pattern: self.pattern.to_string(),
                span: Span::new(position1, position1),
            }
        }
    }

    let result = parser_i.parse_set_class_range();
}


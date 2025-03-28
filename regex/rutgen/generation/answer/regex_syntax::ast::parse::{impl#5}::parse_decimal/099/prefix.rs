// Answer 0

#[test]
fn test_parse_decimal_invalid_case() {
    struct MockParser {
        input: String,
        position: Position,
        is_eof: bool,
        char_index: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.is_eof
        }

        fn char(&self) -> char {
            self.input[self.char_index..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            self.char_index += 1;
        }
        
        fn pos(&self) -> Position {
            self.position
        }
        
        fn scratch(&self) -> RefCell<String> {
            RefCell::new(self.input.clone())
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::DecimalInvalid,
                pattern: self.input.clone(),
                span: Span::new(Position { offset: self.char_index, line: 1, column: 1 }, Position { offset: self.char_index, line: 1, column: 1 }),
            }
        }
    }

    impl Parser {
        fn parse_decimal(&self) -> Result<u32> {
            let mut scratch = self.parser().scratch.borrow_mut();
            scratch.clear();

            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump();
            }
            let span = Span::new(start, self.pos());
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let digits = scratch.as_str();
            if digits.is_empty() {
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    let parser = MockParser {
        input: "0A".to_string(),
        position: Position { offset: 0, line: 1, column: 1 },
        is_eof: false,
        char_index: 0,
    };

    let _result = parser.parse_decimal();
}


// Answer 0

#[test]
fn test_parse_hex_x_bumping_fails() {
    struct MockParser {
        pos: Position,
        input: Vec<char>,
        index: usize,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    impl MockParser {
        fn char(&self) -> char {
            self.input[self.index]
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            false
        }
        
        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: _kind,
                pattern: String::new(),
                span: Span { start: self.pos, end: self.pos },
            }
        }
        
        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }
    }

    let mut parser = MockParser {
        pos: Position::default(),
        input: vec!['x'],
        index: 0,
    };
    
    let result = parser.parse_hex();
}

#[test]
fn test_parse_hex_u_bumping_fails() {
    struct MockParser {
        pos: Position,
        input: Vec<char>,
        index: usize,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    impl MockParser {
        fn char(&self) -> char {
            self.input[self.index]
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            false
        }
        
        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: _kind,
                pattern: String::new(),
                span: Span { start: self.pos, end: self.pos },
            }
        }
        
        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }
    }

    let mut parser = MockParser {
        pos: Position::default(),
        input: vec!['u'],
        index: 0,
    };
    
    let result = parser.parse_hex();
}

#[test]
fn test_parse_hex_U_bumping_fails() {
    struct MockParser {
        pos: Position,
        input: Vec<char>,
        index: usize,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    impl MockParser {
        fn char(&self) -> char {
            self.input[self.index]
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            false
        }
        
        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: _kind,
                pattern: String::new(),
                span: Span { start: self.pos, end: self.pos },
            }
        }
        
        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }
    }

    let mut parser = MockParser {
        pos: Position::default(),
        input: vec!['U'],
        index: 0,
    };
    
    let result = parser.parse_hex();
}


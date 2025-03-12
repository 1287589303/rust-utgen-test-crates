// Answer 0

#[test]
fn test_bump_space_with_whitespace() {
    struct TestParser {
        ignore_whitespace: bool,
        pos: Position,
        pattern: String,
        offset: usize,
    }

    impl TestParser {
        fn new(ignore_whitespace: bool, pattern: &str) -> Self {
            Self {
                ignore_whitespace,
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.offset += self.char().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn bump_space(&mut self) {
            if !self.ignore_whitespace() {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    let start = self.pos();
                    let mut comment_text = String::new();
                    self.bump();
                    while !self.is_eof() {
                        let c = self.char();
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                        comment_text.push(c);
                    }
                    // Simulating comment storage
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new(true, "   # This is a comment\n   x");
    parser.bump_space();
}

#[test]
fn test_bump_space_with_comment_after_whitespace() {
    struct TestParser {
        ignore_whitespace: bool,
        pos: Position,
        pattern: String,
        offset: usize,
    }

    impl TestParser {
        fn new(ignore_whitespace: bool, pattern: &str) -> Self {
            Self {
                ignore_whitespace,
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.offset += self.char().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn bump_space(&mut self) {
            if !self.ignore_whitespace() {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    let start = self.pos();
                    let mut comment_text = String::new();
                    self.bump();
                    while !self.is_eof() {
                        let c = self.char();
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                        comment_text.push(c);
                    }
                    // Simulating comment storage
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new(true, " # This is another comment\n   y");
    parser.bump_space();
}


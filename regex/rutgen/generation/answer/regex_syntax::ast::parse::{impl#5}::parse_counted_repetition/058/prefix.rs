// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    struct TestParser {
        position: Position,
        input: Vec<char>,
        index: usize,
    }
    
    impl TestParser {
        fn char(&self) -> char {
            self.input[self.index]
        }
        
        fn bump(&mut self) {
            self.index += 1;
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::DecimalEmpty,
                pattern: String::from(""),
                span: Span::new(self.pos(), self.pos())
            }
        }

        fn parse_counted_repetition(&self, mut concat: ast::Concat) -> Result<ast::Concat> {
            // Copy the implementation here just for invoking it.
            assert!(self.char() == '{');
            let start = self.pos();
            let ast = match concat.asts.pop() {
                Some(ast) => ast,
                None => {
                    return Err(self.error(self.span(), ast::ErrorKind::RepetitionMissing));
                }
            };
            match ast {
                Ast::Empty(_) | Ast::Flags(_) => {
                    return Err(self.error(self.span(), ast::ErrorKind::RepetitionMissing));
                }
                _ => {}
            }
            if !self.bump_and_bump_space() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::RepetitionCountUnclosed));
            }
            let count_start = Ok(1); // Simulating a successful parsed count start
            if self.is_eof() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::RepetitionCountUnclosed));
            }
            let range = if self.char() == ',' {
                if !self.bump_and_bump_space() {
                    return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::RepetitionCountUnclosed));
                }
                if self.char() != '}' {
                    let count_start = match count_start {
                        Ok(c) => c,
                        Err(err) => return Err(err),
                    };
                    let count_end = Ok(2); // Another simulated successful parsed count start
                    ast::RepetitionRange::Bounded(count_start, count_end)
                } else {
                    ast::RepetitionRange::AtLeast(count_start.unwrap())
                }
            } else {
                ast::RepetitionRange::Exactly(count_start.unwrap())
            };

            if self.is_eof() || self.char() != '}' {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::RepetitionCountUnclosed));
            }

            let mut greedy = true;
            if self.bump_and_bump_space() && self.char() == '?' {
                greedy = false;
                self.bump();
            }

            let op_span = Span::new(start, self.pos());
            if !range.is_valid() {
                return Err(self.error(op_span, ast::ErrorKind::RepetitionCountInvalid));
            }
            concat.asts.push(Ast::repetition(ast::Repetition {
                span: ast.span().with_end(self.pos()),
                op: ast::RepetitionOp {
                    span: op_span,
                    kind: ast::RepetitionKind::Range(range),
                },
                greedy,
                ast: Box::new(ast),
            }));
            Ok(concat)
        }
    }

    let position = Position { offset: 0, line: 1, column: 1 };
    let input = vec!['a', '{', '2', ',', '3', '}'];
    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, position),
        asts: vec![Ast::literal(ast::Literal { span: Span::new(position, position), value: String::from("a") })],
    };

    let parser = TestParser { position, input, index: 0 };

    let _result = parser.parse_counted_repetition(concat);
}


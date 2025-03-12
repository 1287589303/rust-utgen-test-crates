// Answer 0

#[test]
fn test_error_eof_while_parsing_list() {
    struct ReadMock {
        line: usize,
        column: usize,
    }

    impl ReadMock {
        fn new(line: usize, column: usize) -> Self {
            ReadMock { line, column }
        }

        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let read_mock = ReadMock::new(1, 0);
    let reason = ErrorCode::EofWhileParsingList;

    let _result = error(&read_mock, reason);
}

#[test]
fn test_error_expected_colon() {
    struct ReadMock {
        line: usize,
        column: usize,
    }

    impl ReadMock {
        fn new(line: usize, column: usize) -> Self {
            ReadMock { line, column }
        }

        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let read_mock = ReadMock::new(2, 5);
    let reason = ErrorCode::ExpectedColon;

    let _result = error(&read_mock, reason);
}

#[test]
fn test_error_eof_while_parsing_object() {
    struct ReadMock {
        line: usize,
        column: usize,
    }

    impl ReadMock {
        fn new(line: usize, column: usize) -> Self {
            ReadMock { line, column }
        }

        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let read_mock = ReadMock::new(3, 10);
    let reason = ErrorCode::EofWhileParsingObject;

    let _result = error(&read_mock, reason);
}

#[test]
fn test_error_number_out_of_range() {
    struct ReadMock {
        line: usize,
        column: usize,
    }

    impl ReadMock {
        fn new(line: usize, column: usize) -> Self {
            ReadMock { line, column }
        }

        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let read_mock = ReadMock::new(4, 2);
    let reason = ErrorCode::NumberOutOfRange;

    let _result = error(&read_mock, reason);
}

#[test]
fn test_error_expected_some_value() {
    struct ReadMock {
        line: usize,
        column: usize,
    }

    impl ReadMock {
        fn new(line: usize, column: usize) -> Self {
            ReadMock { line, column }
        }

        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let read_mock = ReadMock::new(5, 4);
    let reason = ErrorCode::ExpectedSomeValue;

    let _result = error(&read_mock, reason);
}


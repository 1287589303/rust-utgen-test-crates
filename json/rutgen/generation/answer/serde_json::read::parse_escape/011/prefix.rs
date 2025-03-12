// Answer 0

#[test]
fn test_parse_escape_quote() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'"']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_backslash() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'\\']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_forward_slash() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'/']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_backspace() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'b']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_form_feed() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'f']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_newline() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'n']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_carriage_return() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b'r']);
    let result = parse_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_escape_tab() {
    struct TestReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = TestReader::new(vec![b't']);
    let result = parse_escape(&mut read, false, &mut scratch);
}


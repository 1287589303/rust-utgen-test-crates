// Answer 0

#[test]
fn test_next_or_eof_with_available_byte() {
    struct MockReader {
        bytes: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, pos: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                let byte = self.bytes[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.pos as u64 + 1 }
        }
    }

    let mut reader = MockReader::new(vec![10]);
    let _ = next_or_eof(&mut reader);
}

#[test]
fn test_next_or_eof_with_multiple_available_bytes() {
    struct MockReader {
        bytes: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, pos: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                let byte = self.bytes[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.pos as u64 + 1 }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    let _ = next_or_eof(&mut reader);
    let _ = next_or_eof(&mut reader);
    let _ = next_or_eof(&mut reader);
}

#[test]
#[should_panic]
fn test_next_or_eof_with_empty_input() {
    struct MockReader {
        bytes: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, pos: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                let byte = self.bytes[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.pos as u64 + 1 }
        }
    }

    let mut reader = MockReader::new(vec![]);
    let _ = next_or_eof(&mut reader);
}


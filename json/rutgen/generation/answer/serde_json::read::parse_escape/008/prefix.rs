// Answer 0

#[test]
fn test_parse_escape_valid_backspace() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x62) // hex for 'b'
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'b']);
    let result = parse_escape(&mut reader, true, &mut scratch);
}

#[test]
fn test_parse_escape_valid_newline() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x62) // hex for 'n'
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'n']);
    let result = parse_escape(&mut reader, true, &mut scratch);
}

#[test]
fn test_parse_escape_invalid_character() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x10) // not valid escape
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'x']);
    let result = parse_escape(&mut reader, true, &mut scratch);
}

#[test]
fn test_parse_escape_valid_tab() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x74) // hex for 't'
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b't']);
    let result = parse_escape(&mut reader, true, &mut scratch);
}


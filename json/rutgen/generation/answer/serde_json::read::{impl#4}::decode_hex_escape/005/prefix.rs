// Answer 0

#[test]
fn test_decode_hex_escape_valid_case_1() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }
    
    impl TestRead {
        fn new() -> Self {
            Self {
                input: b"1234".to_vec(),
                position: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    let mut reader = TestRead::new();
    let result = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_valid_case_2() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new() -> Self {
            Self {
                input: b"ABCD".to_vec(),
                position: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    let mut reader = TestRead::new();
    let result = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_valid_case_3() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new() -> Self {
            Self {
                input: b"7F00".to_vec(),
                position: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    let mut reader = TestRead::new();
    let result = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_invalid_case() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new() -> Self {
            Self {
                input: b"XYZ1".to_vec(),
                position: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    let mut reader = TestRead::new();
    let result = reader.decode_hex_escape();
}


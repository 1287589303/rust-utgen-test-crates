// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'"'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_backslash() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'\\'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_forward_slash() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'/'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_backspace() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'b'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_formfeed() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'f'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_newline() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'n'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_carriage_return() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'r'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_horizontal_tab() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b't'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}

#[test]
fn test_parse_escape_invalid() {
    struct ReadMock {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for ReadMock {
        fn next(&mut self) -> Option<&u8> {
            if self.index < self.input.len() {
                let byte = &self.input[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.next().map(|&b| b).ok_or(ErrorCode::EofWhileParsingString.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = ReadMock { input: vec![b'\\', b'x'], index: 0 };
    let _ = parse_escape(&mut reader, false, &mut scratch);
}


// Answer 0

#[test]
fn test_decode_hex_escape_valid_00() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("00");
    let _ = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_valid_ff() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("ff");
    let _ = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_valid_1a() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("1a");
    let _ = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_invalid_gh() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("gh");
    let _ = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_empty() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("");
    let _ = reader.decode_hex_escape();
}

#[test]
fn test_decode_hex_escape_invalid_length_abc() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
            }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = str::from_utf8(&self.input).unwrap(); 
            let value = u16::from_str_radix(hex_str, 16).map_err(|_| Error::custom("Invalid hex"))?;
            Ok(value)
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new("abc");
    let _ = reader.decode_hex_escape();
}


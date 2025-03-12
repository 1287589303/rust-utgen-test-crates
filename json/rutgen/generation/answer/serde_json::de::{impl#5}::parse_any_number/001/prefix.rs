// Answer 0

#[test]
fn test_parse_any_number_with_positive_zero() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"0".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_with_positive_one() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"1".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_with_large_u64() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let large_u64_str = format!("{}", u64::MAX);
    let mut reader = TestReader::new(large_u64_str.as_bytes().to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_with_invalid_input() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }
    
    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"abc".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_with_negative_flag() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"1".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(false);
}

#[test]
fn test_parse_any_number_with_decimal_notation() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"123.456".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_with_scientific_notation() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut reader = TestReader::new(b"1e10".to_vec());
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.parse_any_number(true);
}


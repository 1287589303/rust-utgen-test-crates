// Answer 0

#[test]
fn test_do_deserialize_i128_success_with_negative() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
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
            Position::new(self.position, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> { Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0)) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0)) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut read = MockRead { input: b"   - a123".to_vec(), position: 0 };
    let mut deserializer = Deserializer::new(read);

    let visitor = MockVisitor;
    let _ = deserializer.do_deserialize_i128(visitor);
}

#[test]
fn test_do_deserialize_i128_invalid_number_format() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
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
            Position::new(self.position, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> { Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0)) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0)) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut read = MockRead { input: b"   - 12b".to_vec(), position: 0 };
    let mut deserializer = Deserializer::new(read);

    let visitor = MockVisitor;
    let _ = deserializer.do_deserialize_i128(visitor);
}


// Answer 0

#[test]
fn test_peek_error_eof_while_parsing_list() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let reader = TestReader { position: Position { line: 1, column: 1 } };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.peek_error(ErrorCode::EofWhileParsingList);
}

#[test]
fn test_peek_error_eof_while_parsing_object() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let reader = TestReader { position: Position { line: 2, column: 2 } };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.peek_error(ErrorCode::EofWhileParsingObject);
}

#[test]
fn test_peek_error_eof_while_parsing_string() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let reader = TestReader { position: Position { line: 3, column: 3 } };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.peek_error(ErrorCode::EofWhileParsingString);
}

#[test]
fn test_peek_error_expected_colon() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let reader = TestReader { position: Position { line: 4, column: 4 } };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.peek_error(ErrorCode::ExpectedColon);
}

// Repeat above test structure for each remaining ErrorCode variant in the provided input conditions

#[test]
fn test_peek_error_invalid_number() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let reader = TestReader { position: Position { line: 5, column: 5 } };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.peek_error(ErrorCode::InvalidNumber);
}


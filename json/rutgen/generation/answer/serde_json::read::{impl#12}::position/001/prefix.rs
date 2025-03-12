// Answer 0

#[test]
fn test_position_valid() {
    struct MockReader {
        position: Position,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            unimplemented!()
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockReader { position: Position { line: 1, column: 1 } };
    let pos = mock_reader.position();
}

#[test]
fn test_position_edge_case() {
    struct MockReader {
        position: Position,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            unimplemented!()
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockReader { position: Position { line: usize::MAX, column: usize::MAX } };
    let pos = mock_reader.position();
}


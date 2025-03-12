// Answer 0

#[test]
fn test_eat_char_empty_input() {
    struct EmptyRead;
    impl<'de> Read<'de> for EmptyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: EmptyRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    deserializer.eat_char();
}

#[test]
fn test_eat_char_non_empty_input() {
    struct NonEmptyRead {
        data: Vec<u8>,
        position: usize,
    }
    impl<'de> Read<'de> for NonEmptyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let result = Some(self.data[self.position]);
                self.position += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: NonEmptyRead { data: vec![1, 2, 3], position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    deserializer.eat_char();
}

#[test]
fn test_eat_char_invalid_state() {
    struct InvalidRead;
    impl<'de> Read<'de> for InvalidRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(0)) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(0)) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: InvalidRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    deserializer.eat_char();
}


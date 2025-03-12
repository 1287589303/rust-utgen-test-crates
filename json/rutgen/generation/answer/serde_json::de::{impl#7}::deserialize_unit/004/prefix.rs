// Answer 0

#[test]
fn test_deserialize_unit_valid_whitespace() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct DummyRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed(&"dummy" as &str))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(&b"dummy"[..]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = DummyRead { input: b" \tnull".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.deserialize_unit(DummyVisitor);
}

#[test]
fn test_deserialize_unit_invalid_whitespace() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct DummyRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed(&"dummy" as &str))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(&b"dummy"[..]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = DummyRead { input: b" \txyz".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.deserialize_unit(DummyVisitor);
}

#[test]
fn test_deserialize_unit_unexpected_character() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = ();
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct DummyRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0) // Dummy Position
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed(&"dummy" as &str))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(&b"dummy"[..]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = DummyRead { input: b"n".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.deserialize_unit(DummyVisitor);
}


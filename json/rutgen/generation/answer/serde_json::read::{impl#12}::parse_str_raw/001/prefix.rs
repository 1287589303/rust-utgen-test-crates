// Answer 0

#[test]
fn test_parse_str_raw_valid_input() {
    struct ValidReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for ValidReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(&scratch[..]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = ValidReader {
        data: b"valid input".to_vec(),
        position: 0,
    };
    let mut scratch = vec![b'a', b'b', b'c'];

    let _result = reader.parse_str_raw(&mut scratch);
}

#[test]
fn test_parse_str_raw_empty_scratch() {
    struct InvalidReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for InvalidReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::custom("Scratch vector is empty"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = InvalidReader {
        data: b"invalid input".to_vec(),
        position: 0,
    };
    let mut scratch = vec![];

    let _result = reader.parse_str_raw(&mut scratch);
}


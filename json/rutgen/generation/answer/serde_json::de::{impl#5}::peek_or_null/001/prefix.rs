// Answer 0

#[test]
fn test_peek_or_null_empty_stream() {
    struct EmptyReader;
    impl<'de> Read<'de> for EmptyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Err(Error::custom("EOF reached"))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: EmptyReader,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let _result = deserializer.peek_or_null();
}

#[test]
fn test_peek_or_null_corrupted_format() {
    struct CorruptedReader;
    impl<'de> Read<'de> for CorruptedReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Err(Error::custom("Corrupted format"))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: CorruptedReader,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let _result = deserializer.peek_or_null();
}


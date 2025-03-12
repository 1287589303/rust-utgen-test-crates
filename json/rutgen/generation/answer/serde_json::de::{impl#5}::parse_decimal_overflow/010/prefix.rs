// Answer 0

#[test]
fn test_parse_decimal_overflow_case1() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            todo!()
        }
    }

    let reader = TestReader {
        data: b"1.23e308".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let positive = true;
    let significand = 99999999999999999999;
    let exponent = 308;

    deserializer.parse_decimal_overflow(positive, significand, exponent).unwrap();
}

#[test]
fn test_parse_decimal_overflow_case2() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            todo!()
        }
    }

    let reader = TestReader {
        data: b"2.34E308".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let positive = true;
    let significand = 99999999999999999999;
    let exponent = 308;

    deserializer.parse_decimal_overflow(positive, significand, exponent).unwrap();
}


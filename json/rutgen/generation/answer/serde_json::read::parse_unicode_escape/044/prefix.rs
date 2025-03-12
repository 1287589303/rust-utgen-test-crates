// Answer 0

#[test]
fn test_parse_unicode_escape_leading_surrogate() {
    struct MockRead {
        hex_value: Result<u16>,
        peek_value: Result<u8>,
        discard_called: bool,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                hex_value: Ok(0xD800),
                peek_value: Ok(b'\\'),
                discard_called: false,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_value.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(self.peek_value.clone().unwrap()))
        }

        fn discard(&mut self) {
            self.discard_called = true;
        }
    }

    let mut mock_read = MockRead::new();
    let validate = false;
    let mut scratch = Vec::new();

    let _ = parse_unicode_escape(&mut mock_read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_trailing_surrogate() {
    struct MockRead {
        hex_value: Result<u16>,
        peek_values: Vec<Result<u8>>,
        discard_called: bool,
        peek_index: usize,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                hex_value: Ok(0xDBFF),
                peek_values: vec![Ok(b'\\'), Ok(b'u')],
                discard_called: false,
                peek_index: 0,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_value.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.peek_index < self.peek_values.len() {
                let value = self.peek_values[self.peek_index].clone();
                self.peek_index += 1;
                Ok(Some(value.unwrap()))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.discard_called = true;
        }
    }

    let mut mock_read = MockRead::new();
    let validate = false;
    let mut scratch = Vec::new();

    let _ = parse_unicode_escape(&mut mock_read, validate, &mut scratch);
}


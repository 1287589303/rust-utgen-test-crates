// Answer 0

#[test]
fn test_variant_seed_should_fail() {
    struct FailingSeed;

    impl<'de> de::DeserializeSeed<'de> for FailingSeed {
        type Value = ();

        fn deserialize<React>(&self, deserializer: &mut React) -> Result<Self::Value>
        where
            React: de::Deserializer<'de>,
        {
            Err(Error)
        }
    }

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let val = self.data[self.position];
                self.position += 1;
                Ok(Some(val))
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

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error)
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error)
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error)
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b'{' , b'}'], position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let variant_access = VariantAccess { de: &mut deserializer };
    let result = variant_access.variant_seed(FailingSeed);
}


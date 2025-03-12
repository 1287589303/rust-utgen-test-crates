// Answer 0

#[test]
fn test_try_get_i16_le_success() {
    struct BufImpl<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for BufImpl<'a> {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            Ok(i16::from_le_bytes(bytes))
        }
    }

    let mut buf = BufImpl { data: &b"\x09\x08"[..], position: 0 };
    let result = buf.try_get_i16_le();
}

#[test]
fn test_try_get_i16_le_failure() {
    struct BufImpl<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for BufImpl<'a> {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let bytes = [self.chunk()[0], self.chunk()[1]];
            self.advance(2);
            Ok(i16::from_le_bytes(bytes))
        }
    }

    let mut buf = BufImpl { data: &b"\x08"[..], position: 0 };
    let result = buf.try_get_i16_le();
}


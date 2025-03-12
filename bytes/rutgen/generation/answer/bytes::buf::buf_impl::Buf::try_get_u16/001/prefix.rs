// Answer 0

#[test]
fn test_try_get_u16_with_no_bytes_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u16(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                Err(TryGetError { requested: 2, available: self.remaining() })
            } else {
                let bytes = [self.data[self.position], self.data[self.position + 1]];
                self.position += 2;
                Ok(u16::from_be_bytes(bytes))
            }
        }
    }

    let mut buf = TestBuf { data: &b""[..], position: 0 };
    let result = buf.try_get_u16();
    assert_eq!(result, Err(TryGetError { requested: 2, available: 0 }));
}

#[test]
fn test_try_get_u16_with_one_byte_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u16(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                Err(TryGetError { requested: 2, available: self.remaining() })
            } else {
                let bytes = [self.data[self.position], self.data[self.position + 1]];
                self.position += 2;
                Ok(u16::from_be_bytes(bytes))
            }
        }
    }

    let mut buf = TestBuf { data: &b"\x01"[..], position: 0 };
    let result = buf.try_get_u16();
    assert_eq!(result, Err(TryGetError { requested: 2, available: 1 }));
}


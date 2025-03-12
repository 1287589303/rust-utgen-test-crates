// Answer 0

#[test]
fn test_try_get_u32_success() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let ret = u32::from_be_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
            self.pos += 4;
            Ok(ret)
        }
    }

    let mut buf = TestBuf { data: &b"\x08\x09\xA0\xA1"[..], pos: 0 };
    assert_eq!(Ok(0x0809A0A1), buf.try_get_u32());
}

#[test]
fn test_try_get_u32_failure() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let ret = u32::from_be_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
            self.pos += 4;
            Ok(ret)
        }
    }

    let mut buf = TestBuf { data: &b"\x01\x02\x03"[..], pos: 0 };
    assert_eq!(Err(TryGetError { requested: 4, available: 3 }), buf.try_get_u32());
}


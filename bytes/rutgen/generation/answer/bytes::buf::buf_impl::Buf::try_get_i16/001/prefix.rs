// Answer 0

#[test]
fn test_try_get_i16_insufficient_bytes_0() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let bytes = [self.data[self.position], self.data[self.position + 1]];
            self.advance(2);
            Ok(i16::from_be_bytes(bytes))
        }
    }

    let mut buf = TestBuf { data: &[], position: 0 };
    assert_eq!(buf.try_get_i16(), Err(TryGetError { requested: 2, available: 0 }));
}

#[test]
fn test_try_get_i16_insufficient_bytes_1() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let bytes = [self.data[self.position], self.data[self.position + 1]];
            self.advance(2);
            Ok(i16::from_be_bytes(bytes))
        }
    }

    let mut buf = TestBuf { data: &b"\x08"[..], position: 0 };
    assert_eq!(buf.try_get_i16(), Err(TryGetError { requested: 2, available: 1 }));
}

#[test]
fn test_try_get_i16_sufficient_bytes() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let bytes = [self.data[self.position], self.data[self.position + 1]];
            self.advance(2);
            Ok(i16::from_be_bytes(bytes))
        }
    }

    let mut buf = TestBuf { data: &b"\x08\x09"[..], position: 0 };
    assert_eq!(buf.try_get_i16(), Ok(0x0809));
}


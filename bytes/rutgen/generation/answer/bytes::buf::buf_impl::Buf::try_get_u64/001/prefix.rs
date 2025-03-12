// Answer 0

#[test]
fn test_try_get_u64_insufficient_bytes_1() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let mut buf = [0; 8];
            buf.copy_from_slice(&self.data[self.position..self.position + 8]);
            self.position += 8;
            Ok(u64::from_be_bytes(buf))
        }
    }

    let mut buf = TestBuf {
        data: b"\x01\x02\x03\x04\x05\x06\x07"[..].to_vec(),
        position: 0,
    };
    assert_eq!(Err(TryGetError { requested: 8, available: 7 }), buf.try_get_u64());
    assert_eq!(7, buf.remaining());
}

#[test]
fn test_try_get_u64_insufficient_bytes_2() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let mut buf = [0; 8];
            buf.copy_from_slice(&self.data[self.position..self.position + 8]);
            self.position += 8;
            Ok(u64::from_be_bytes(buf))
        }
    }

    let mut buf = TestBuf {
        data: b"\x01\x02\x03\x04\x05\x06"[..].to_vec(),
        position: 0,
    };
    assert_eq!(Err(TryGetError { requested: 8, available: 6 }), buf.try_get_u64());
    assert_eq!(6, buf.remaining());
}

#[test]
fn test_try_get_u64_no_bytes() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let mut buf = [0; 8];
            buf.copy_from_slice(&self.data[self.position..self.position + 8]);
            self.position += 8;
            Ok(u64::from_be_bytes(buf))
        }
    }

    let mut buf = TestBuf {
        data: b""[..].to_vec(),
        position: 0,
    };
    assert_eq!(Err(TryGetError { requested: 8, available: 0 }), buf.try_get_u64());
    assert_eq!(0, buf.remaining());
}


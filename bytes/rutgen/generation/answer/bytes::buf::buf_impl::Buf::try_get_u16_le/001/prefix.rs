// Answer 0

#[test]
fn test_try_get_u16_le_not_enough_bytes_available_0() {
    struct TestBuf {
        buf: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(buf: Vec<u8>) -> Self {
            Self { buf, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.buf.len() - self.position
        }

        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let value = u16::from_le_bytes([self.buf[self.position], self.buf[self.position + 1]]);
            self.position += 2;
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0x08]);
    let result = buf.try_get_u16_le();
}

#[test]
fn test_try_get_u16_le_not_enough_bytes_available_1() {
    struct TestBuf {
        buf: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(buf: Vec<u8>) -> Self {
            Self { buf, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.buf.len() - self.position
        }

        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let value = u16::from_le_bytes([self.buf[self.position], self.buf[self.position + 1]]);
            self.position += 2;
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![]);
    let result = buf.try_get_u16_le();
}


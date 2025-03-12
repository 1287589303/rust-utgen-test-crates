// Answer 0

#[test]
fn test_try_get_i32_le_with_no_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let value = i32::from_le_bytes([
                self.data[self.position],
                self.data[self.position + 1],
                self.data[self.position + 2],
                self.data[self.position + 3],
            ]);
            self.position += 4;
            Ok(value)
        }
    }

    let mut buf = TestBuf {
        data: &b""[..],
        position: 0,
    };
    let result = buf.try_get_i32_le();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 0 }));
}

#[test]
fn test_try_get_i32_le_with_one_byte_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let value = i32::from_le_bytes([
                self.data[self.position],
                self.data[self.position + 1],
                self.data[self.position + 2],
                self.data[self.position + 3],
            ]);
            self.position += 4;
            Ok(value)
        }
    }

    let mut buf = TestBuf {
        data: &b"\x01"[..],
        position: 0,
    };
    let result = buf.try_get_i32_le();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 1 }));
}

#[test]
fn test_try_get_i32_le_with_two_bytes_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let value = i32::from_le_bytes([
                self.data[self.position],
                self.data[self.position + 1],
                self.data[self.position + 2],
                self.data[self.position + 3],
            ]);
            self.position += 4;
            Ok(value)
        }
    }

    let mut buf = TestBuf {
        data: &b"\x01\x02"[..],
        position: 0,
    };
    let result = buf.try_get_i32_le();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 2 }));
}

#[test]
fn test_try_get_i32_le_with_three_bytes_remaining() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> {
            if self.remaining() < 4 {
                return Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                });
            }
            let value = i32::from_le_bytes([
                self.data[self.position],
                self.data[self.position + 1],
                self.data[self.position + 2],
                self.data[self.position + 3],
            ]);
            self.position += 4;
            Ok(value)
        }
    }

    let mut buf = TestBuf {
        data: &b"\x01\x02\x03"[..],
        position: 0,
    };
    let result = buf.try_get_i32_le();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 3 }));
}


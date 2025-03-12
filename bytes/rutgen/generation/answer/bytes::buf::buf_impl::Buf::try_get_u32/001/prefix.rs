// Answer 0

#[test]
fn test_try_get_u32_insufficient_bytes_0() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                })
            } else {
                let value = u32::from_be_bytes([
                    self.data[self.position],
                    self.data[self.position + 1],
                    self.data[self.position + 2],
                    self.data[self.position + 3],
                ]);
                self.position += 4;
                Ok(value)
            }
        }
    }

    let mut buf = TestBuf { data: &[1, 2, 3][..], position: 0 };
    let result = buf.try_get_u32();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 3 }));
}

#[test]
fn test_try_get_u32_insufficient_bytes_1() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                })
            } else {
                let value = u32::from_be_bytes([
                    self.data[self.position],
                    self.data[self.position + 1],
                    self.data[self.position + 2],
                    self.data[self.position + 3],
                ]);
                self.position += 4;
                Ok(value)
            }
        }
    }

    let mut buf = TestBuf { data: &[1, 2][..], position: 0 };
    let result = buf.try_get_u32();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 2 }));
}

#[test]
fn test_try_get_u32_insufficient_bytes_2() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                })
            } else {
                let value = u32::from_be_bytes([
                    self.data[self.position],
                    self.data[self.position + 1],
                    self.data[self.position + 2],
                    self.data[self.position + 3],
                ]);
                self.position += 4;
                Ok(value)
            }
        }
    }

    let mut buf = TestBuf { data: &[0][..], position: 0 };
    let result = buf.try_get_u32();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 1 }));
}

#[test]
fn test_try_get_u32_insufficient_bytes_empty() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            if self.remaining() < 4 {
                Err(TryGetError {
                    requested: 4,
                    available: self.remaining(),
                })
            } else {
                let value = u32::from_be_bytes([
                    self.data[self.position],
                    self.data[self.position + 1],
                    self.data[self.position + 2],
                    self.data[self.position + 3],
                ]);
                self.position += 4;
                Ok(value)
            }
        }
    }

    let mut buf = TestBuf { data: &[][..], position: 0 };
    let result = buf.try_get_u32();
    assert_eq!(result, Err(TryGetError { requested: 4, available: 0 }));
}


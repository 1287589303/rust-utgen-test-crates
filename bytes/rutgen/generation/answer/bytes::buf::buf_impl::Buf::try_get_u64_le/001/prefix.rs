// Answer 0

#[test]
fn test_try_get_u64_le_with_available_7_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08\x07\x06\x05\x04\x03\x02", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_6_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08\x07\x06\x05\x04\x03", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_5_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08\x07\x06\x05", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_4_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08\x07\x06", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_3_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08\x07", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_2_bytes() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"\x08", offset: 0 };
    let result = buf.try_get_u64_le();
}

#[test]
fn test_try_get_u64_le_with_available_1_byte() {
    struct TestBuf {
        data: &'static [u8],
        offset: usize,
    }
    
    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.offset
        }

        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError {
                    requested: 8,
                    available: self.remaining(),
                });
            }
            let bytes = &self.data[self.offset..self.offset + 8];
            self.offset += 8;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }

    let mut buf = TestBuf { data: b"", offset: 0 };
    let result = buf.try_get_u64_le();
}


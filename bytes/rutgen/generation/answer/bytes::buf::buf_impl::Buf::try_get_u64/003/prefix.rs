// Answer 0

#[cfg(test)]
fn test_try_get_u64_success() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            Self { data, pos: 0 }
        }
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError { requested: 8, available: self.remaining() });
            }
            let ret = u64::from_be_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
            self.pos += 8;
            Ok(ret)
        }
    }

    let mut buf = TestBuf::new(&b"\x01\x02\x03\x04\x05\x06\x07\x08 hello"[..]);
    let _ = buf.try_get_u64();
}

#[cfg(test)]
fn test_try_get_u64_failure() {
    struct TestBuf {
        data: &'static [u8],
        pos: usize,
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            Self { data, pos: 0 }
        }
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            if self.remaining() < 8 {
                return Err(TryGetError { requested: 8, available: self.remaining() });
            }
            let ret = u64::from_be_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
            self.pos += 8;
            Ok(ret)
        }
    }

    let mut buf = TestBuf::new(&b"\x01\x02\x03\x04\x05\x06\x07"[..]);
    let _ = buf.try_get_u64();
}


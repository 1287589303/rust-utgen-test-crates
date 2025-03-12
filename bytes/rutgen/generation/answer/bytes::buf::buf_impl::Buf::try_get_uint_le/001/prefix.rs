// Answer 0

#[test]
fn test_try_get_uint_le_valid_1_byte() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, position: 0 }
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.chunk()[..nbytes].iter().fold(0u64, |acc, &b| acc | (b as u64) << (8 * acc.count_ones() as usize));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0x01]);
    let _ = buf.try_get_uint_le(1);
}

#[test]
fn test_try_get_uint_le_valid_2_bytes() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, position: 0 }
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.chunk()[..nbytes].iter().fold(0u64, |acc, &b| acc | (b as u64) << (8 * acc.count_ones() as usize));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0x01, 0x02]);
    let _ = buf.try_get_uint_le(2);
}

#[test]
fn test_try_get_uint_le_valid_3_bytes() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, position: 0 }
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.chunk()[..nbytes].iter().fold(0u64, |acc, &b| acc | (b as u64) << (8 * acc.count_ones() as usize));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0x01, 0x02, 0x03]);
    let _ = buf.try_get_uint_le(3);
}

#[test]
fn test_try_get_uint_le_insufficient_bytes() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, position: 0 }
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.chunk()[..nbytes].iter().fold(0u64, |acc, &b| acc | (b as u64) << (8 * acc.count_ones() as usize));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0x01, 0x02]);
    let _ = buf.try_get_uint_le(4);
}

#[test]
#[should_panic]
fn test_try_get_uint_le_panic_on_too_many_bytes() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, position: 0 }
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.chunk()[..nbytes].iter().fold(0u64, |acc, &b| acc | (b as u64) << (8 * acc.count_ones() as usize));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf::new(vec![0; 4]);
    let _ = buf.try_get_uint_le(9);
}


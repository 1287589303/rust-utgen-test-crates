// Answer 0

#[test]
fn test_get_i64_ne_valid_big_endian() {
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
        
        fn get_i64_ne(&mut self) -> i64 {
            if self.remaining() < 8 {
                panic!("not enough remaining data");
            }
            let bytes: [u8; 8] = self.data[self.position..self.position + 8]
                .try_into().unwrap();
            self.position += 8;
            i64::from_ne_bytes(bytes)
        }
    }

    let mut buf = TestBuf::new(b"\x01\x02\x03\x04\x05\x06\x07\x08".to_vec());
    let value = buf.get_i64_ne();
}

#[test]
fn test_get_i64_ne_valid_little_endian() {
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
        
        fn get_i64_ne(&mut self) -> i64 {
            if self.remaining() < 8 {
                panic!("not enough remaining data");
            }
            let bytes: [u8; 8] = self.data[self.position..self.position + 8]
                .try_into().unwrap();
            self.position += 8;
            i64::from_ne_bytes(bytes)
        }
    }

    let mut buf = TestBuf::new(b"\x08\x07\x06\x05\x04\x03\x02\x01".to_vec());
    let value = buf.get_i64_ne();
}

#[test]
#[should_panic]
fn test_get_i64_ne_insufficient_data() {
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
        
        fn get_i64_ne(&mut self) -> i64 {
            if self.remaining() < 8 {
                panic!("not enough remaining data");
            }
            let bytes: [u8; 8] = self.data[self.position..self.position + 8]
                .try_into().unwrap();
            self.position += 8;
            i64::from_ne_bytes(bytes)
        }
    }

    let mut buf = TestBuf::new(b"\x01\x02\x03\x04\x05\x06\x07".to_vec());
    let value = buf.get_i64_ne();
}


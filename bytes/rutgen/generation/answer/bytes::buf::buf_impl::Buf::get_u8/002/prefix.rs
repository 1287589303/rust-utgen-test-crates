// Answer 0

#[test]
fn test_get_u8_valid() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
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
    }

    let mut buf = TestBuf::new(vec![8]);
    let result = buf.get_u8();
}

#[test]
#[should_panic]
fn test_get_u8_no_remaining() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
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
    }

    let mut buf = TestBuf::new(vec![]);
    let _ = buf.get_u8();
}


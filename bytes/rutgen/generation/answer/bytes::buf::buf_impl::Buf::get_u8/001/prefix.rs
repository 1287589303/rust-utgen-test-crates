// Answer 0

#[test]
#[should_panic]
fn test_get_u8_empty_buffer() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
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

    let mut buf = TestBuf { data: &[], position: 0 };
    buf.get_u8();
}

#[test]
#[should_panic]
fn test_get_u8_insufficient_data() {
    struct TestBuf {
        data: &'static [u8],
        position: usize,
    }

    impl TestBuf {
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

    let mut buf = TestBuf { data: &[], position: 1 };
    buf.get_u8();
}


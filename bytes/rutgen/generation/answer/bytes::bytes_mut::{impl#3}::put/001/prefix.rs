// Answer 0

#[test]
fn test_put_with_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let mut bytes_mut = BytesMut::new();
    let buf = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        pos: 0,
    };
    
    bytes_mut.put(buf);
}

#[test]
fn test_put_with_no_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let mut bytes_mut = BytesMut::new();
    let buf = TestBuf {
        data: vec![], // No remaining data
        pos: 0,
    };
    
    bytes_mut.put(buf);
}


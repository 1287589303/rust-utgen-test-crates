// Answer 0

#[test]
fn test_advance_with_limit_zero() {
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

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3], pos: 0 };
    let mut take = Take { inner: buf, limit: 0 };
    take.advance(0);
}

#[test]
fn test_advance_with_limit_ten() {
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

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], pos: 0 };
    let mut take = Take { inner: buf, limit: 10 };
    take.advance(10);
}

#[test]
fn test_advance_with_limit_high() {
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

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], pos: 5 };
    let mut take = Take { inner: buf, limit: 5 };
    take.advance(5);
}


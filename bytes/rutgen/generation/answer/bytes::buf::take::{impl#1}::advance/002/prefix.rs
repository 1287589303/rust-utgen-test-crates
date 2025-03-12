// Answer 0

#[test]
#[should_panic]
fn test_advance_exceeds_limit() {
    struct TestBuf {
        limit: usize,
        advanced: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.limit - self.advanced
        }
        
        fn chunk(&self) -> &[u8] {
            &[]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.advanced += cnt;
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let limit = 5;
    let mut test_buf = TestBuf { limit, advanced: 0 };
    let mut take = Take { inner: test_buf, limit };

    let cnt = limit + 1; // cnt exceeds limit
    take.advance(cnt);
}

#[test]
#[should_panic]
fn test_advance_exceeds_limit_with_zero_limit() {
    struct TestBuf {
        limit: usize,
        advanced: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.limit - self.advanced
        }
        
        fn chunk(&self) -> &[u8] {
            &[]
        }
        
        fn advance(&mut self, cnt: usize) {
            self.advanced += cnt;
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
        
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let limit = 0;
    let mut test_buf = TestBuf { limit, advanced: 0 };
    let mut take = Take { inner: test_buf, limit };

    let cnt = limit + 1; // cnt exceeds limit
    take.advance(cnt);
}


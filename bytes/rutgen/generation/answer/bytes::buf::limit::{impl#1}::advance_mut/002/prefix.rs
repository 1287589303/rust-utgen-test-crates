// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_exceeds_limit() {
    struct TestBufMut {
        limit: usize,
        advanced: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.limit - self.advanced
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.advanced += cnt;
        }
    }

    let mut buf = TestBufMut { limit: 5, advanced: 0 };

    let mut limit_buf = Limit { inner: buf, limit: 3 };

    unsafe {
        limit_buf.advance_mut(4);
    }
}

#[test]
#[should_panic]
fn test_advance_mut_zero_limit() {
    struct TestBufMut {
        limit: usize,
        advanced: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.limit - self.advanced
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.advanced += cnt;
        }
    }

    let mut buf = TestBufMut { limit: 0, advanced: 0 };

    let mut limit_buf = Limit { inner: buf, limit: 0 };

    unsafe {
        limit_buf.advance_mut(1);
    }
}


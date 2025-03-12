// Answer 0

#[test]
fn test_put_int_panic_nbytes_greater_than_size_of_val() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                pos: 0,
            }
        }
        
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(10);
    let n = -1i64;
    let nbytes = 9;

    // This should panic
    buf.put_int(n, nbytes);
}

#[test]
fn test_put_int_panic_nbytes_exceeds_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                pos: 0,
            }
        }
        
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(5);
    let n = -128i64;
    let nbytes = 6;

    // This should panic due to insufficient capacity
    buf.put_int(n, nbytes);
}

#[test]
fn test_put_int_successful_case() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                pos: 0,
            }
        }
        
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
        
        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
        
        fn assert_eq_data(&self, expected: &[u8]) {
            assert_eq!(self.data, expected);
        }
    }

    let mut buf = TestBuf::new(10);
    let n = 0x0504010203i64;
    let nbytes = 3;

    buf.put_int(n, nbytes);
    buf.assert_eq_data(&[0x01, 0x02, 0x03, 0, 0, 0, 0, 0, 0, 0]);
}


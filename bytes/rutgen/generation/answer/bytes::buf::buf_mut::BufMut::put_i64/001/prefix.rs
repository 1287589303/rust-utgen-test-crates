// Answer 0

#[test]
fn test_put_i64_valid() {
    struct TestBuf {
        buf: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buf: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.buf.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.buf[self.pos..self.pos + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_i64(0x0102030405060708);
}

#[test]
fn test_put_i64_min_value() {
    struct TestBuf {
        buf: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buf: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.buf.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.buf[self.pos..self.pos + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_i64(i64::MIN);
}

#[test]
fn test_put_i64_max_value() {
    struct TestBuf {
        buf: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buf: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.buf.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.buf[self.pos..self.pos + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_i64(i64::MAX);
}

#[test]
#[should_panic]
fn test_put_i64_not_enough_capacity() {
    struct TestBuf {
        buf: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                buf: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.buf.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.buf[self.pos..self.pos + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(7);
    buf.put_i64(0x0102030405060708);
}


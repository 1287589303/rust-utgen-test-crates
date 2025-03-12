// Answer 0

#[test]
fn test_put_int_with_exact_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.cursor..self.cursor + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_int(0x0504010203, 3);
}

#[test]
fn test_put_int_with_boundary_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.cursor..self.cursor + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_int(0x0504010203, 8);
}

#[test]
#[should_panic]
fn test_put_int_with_overflow_nbytes() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.cursor..self.cursor + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(8);
    buf.put_int(0x0504010203, 9);
}

#[test]
#[should_panic]
fn test_put_int_with_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                cursor: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len();
            self.data[self.cursor..self.cursor + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(2);
    buf.put_int(0x0504010203, 3);
}


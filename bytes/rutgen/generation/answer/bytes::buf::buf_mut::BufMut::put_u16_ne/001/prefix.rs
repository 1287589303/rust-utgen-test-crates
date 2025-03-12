// Answer 0

#[test]
fn test_put_u16_ne_min_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("not enough space");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(0);
    let n: u16 = 0x0809;
    // This should panic because of insufficient capacity
    let result = std::panic::catch_unwind(|| {
        buf.put_slice(&n.to_ne_bytes())
    });
    assert!(result.is_err());
}

#[test]
fn test_put_u16_ne_one_byte_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("not enough space");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(1);
    let n: u16 = 0x0809;
    // This should panic because of insufficient capacity
    let result = std::panic::catch_unwind(|| {
        buf.put_slice(&n.to_ne_bytes())
    });
    assert!(result.is_err());
}

#[test]
fn test_put_u16_ne_exact_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("not enough space");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(2);
    let n: u16 = 0x0809;
    buf.put_slice(&n.to_ne_bytes());
}

#[test]
fn test_put_u16_ne_with_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            TestBuf {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.resize(self.data.len() + cnt, 0);
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("not enough space");
            }
            self.data.extend_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf::new(10);
    let n: u16 = 0x0809;
    buf.put_slice(&n.to_ne_bytes());
}


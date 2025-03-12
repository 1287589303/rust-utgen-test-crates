// Answer 0

#[test]
fn test_put_f32_ne_with_min_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: Vec::with_capacity(capacity),
                pos: 0,
            }
        }

        fn put_f32_ne(&mut self, n: f32) {
            if self.buffer.len() - self.pos < 4 {
                panic!("not enough remaining capacity");
            }
            let bits = n.to_bits();
            self.buffer.extend(&bits.to_ne_bytes());
            self.pos += 4;
        }
    }

    let mut buf = TestBuf::new(4); // exact capacity
    buf.put_f32_ne(1.2f32);
}

#[test]
fn test_put_f32_ne_with_max_value() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: Vec::with_capacity(capacity),
                pos: 0,
            }
        }

        fn put_f32_ne(&mut self, n: f32) {
            if self.buffer.len() - self.pos < 4 {
                panic!("not enough remaining capacity");
            }
            let bits = n.to_bits();
            self.buffer.extend(&bits.to_ne_bytes());
            self.pos += 4;
        }
    }

    let mut buf = TestBuf::new(8); // enough capacity
    buf.put_f32_ne(3.40282347E+38);
}

#[test]
fn test_put_f32_ne_with_min_value() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: Vec::with_capacity(capacity),
                pos: 0,
            }
        }

        fn put_f32_ne(&mut self, n: f32) {
            if self.buffer.len() - self.pos < 4 {
                panic!("not enough remaining capacity");
            }
            let bits = n.to_bits();
            self.buffer.extend(&bits.to_ne_bytes());
            self.pos += 4;
        }
    }

    let mut buf = TestBuf::new(8); // enough capacity
    buf.put_f32_ne(-3.40282347E+38);
}

#[should_panic]
#[test]
fn test_put_f32_ne_with_insufficient_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: Vec::with_capacity(capacity),
                pos: 0,
            }
        }

        fn put_f32_ne(&mut self, n: f32) {
            if self.buffer.len() - self.pos < 4 {
                panic!("not enough remaining capacity");
            }
            let bits = n.to_bits();
            self.buffer.extend(&bits.to_ne_bytes());
            self.pos += 4;
        }
    }

    let mut buf = TestBuf::new(3); // insufficient capacity
    buf.put_f32_ne(1.2f32);
}


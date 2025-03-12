// Answer 0

#[test]
fn test_put_f64_ne_valid() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne(1.2f64.to_bits());
}

#[test]
fn test_put_f64_ne_zero() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne(0.0f64.to_bits());
}

#[test]
fn test_put_f64_ne_negative_zero() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne((-0.0f64).to_bits());
}

#[test]
fn test_put_f64_ne_nan() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne(f64::NAN.to_bits());
}

#[test]
fn test_put_f64_ne_infinity() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne(f64::INFINITY.to_bits());
}

#[test]
fn test_put_f64_ne_negative_infinity() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { buffer: vec![0; capacity], position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_u64_ne(&mut self, n: u64) {
            let bytes = n.to_ne_bytes();
            self.buffer[self.position..self.position + 8].copy_from_slice(&bytes);
            unsafe { self.advance_mut(8) };
        }
    }

    let mut buf = TestBuf::new(16);
    assert!(buf.remaining_mut() >= 8);
    buf.put_u64_ne((-f64::INFINITY).to_bits());
}


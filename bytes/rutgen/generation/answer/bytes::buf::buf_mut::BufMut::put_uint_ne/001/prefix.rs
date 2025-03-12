// Answer 0

#[test]
fn test_put_uint_ne_small_value() {
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

        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
            let start = match 8.checked_sub(nbytes) {
                Some(start) => start,
                None => panic!(),
            };
            let bytes = &n.to_be_bytes()[start..];
            self.data.extend_from_slice(bytes);
            unsafe { self.advance_mut(nbytes) };
        }
    }

    let mut buf = TestBuf::new(8);
    unsafe {
        buf.put_uint_ne(0x01, 1);
    }
}

#[test]
fn test_put_uint_ne_max_value() {
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

        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
            let start = match 8.checked_sub(nbytes) {
                Some(start) => start,
                None => panic!(),
            };
            let bytes = &n.to_be_bytes()[start..];
            self.data.extend_from_slice(bytes);
            unsafe { self.advance_mut(nbytes) };
        }
    }

    let mut buf = TestBuf::new(8);
    unsafe {
        buf.put_uint_ne(0xFFFFFFFFFFFFFFFF, 8);
    }
}

#[test]
fn test_put_uint_ne_boundary_conditions() {
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

        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
            let start = match 8.checked_sub(nbytes) {
                Some(start) => start,
                None => panic!(),
            };
            let bytes = &n.to_be_bytes()[start..];
            self.data.extend_from_slice(bytes);
            unsafe { self.advance_mut(nbytes) };
        }
    }

    let mut buf = TestBuf::new(8);
    unsafe {
        buf.put_uint_ne(0x01, 1); // Smallest nbytes
        buf.put_uint_ne(0x01020304, 4); // Mid range nbytes
        buf.put_uint_ne(0xFFFFFFFFFFFFFFFF, 8); // Largest nbytes
    }
}

#[test]
#[should_panic]
fn test_put_uint_ne_insufficient_capacity() {
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

        fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
            if nbytes > self.remaining_mut() {
                panic!();
            }
            let start = match 8.checked_sub(nbytes) {
                Some(start) => start,
                None => panic!(),
            };
            let bytes = &n.to_be_bytes()[start..];
            self.data.extend_from_slice(bytes);
            unsafe { self.advance_mut(nbytes) };
        }
    }

    let mut buf = TestBuf::new(4); // Insufficient capacity
    unsafe {
        buf.put_uint_ne(0x01, 5); // Should panic
    }
}


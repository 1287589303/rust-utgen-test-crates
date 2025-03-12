// Answer 0

#[test]
fn test_put_u32_ne_min_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                position: 0,
            }
        }
    
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.data[self.position..self.position + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(10);
    unsafe {
        buf.put_u32_ne(0); // Testing with the minimum value of u32
    }
}

#[test]
fn test_put_u32_ne_max_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                position: 0,
            }
        }
    
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.data[self.position..self.position + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(10);
    unsafe {
        buf.put_u32_ne(4294967295); // Testing with the maximum value of u32
    }
}

#[test]
fn test_put_u32_ne_middle_value() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                position: 0,
            }
        }
    
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.data[self.position..self.position + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(10);
    unsafe {
        buf.put_u32_ne(2147483648); // Testing with a middle value of u32
    }
}

#[test]
fn test_put_u32_ne_exact_capacity_remaining() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                position: 0,
            }
        }
    
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = usize::min(src.len(), self.remaining_mut());
            self.data[self.position..self.position + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) };
        }
    }

    let mut buf = TestBuf::new(4);
    unsafe {
        buf.put_u32_ne(123456); // Testing with exact remaining capacity
    }
}


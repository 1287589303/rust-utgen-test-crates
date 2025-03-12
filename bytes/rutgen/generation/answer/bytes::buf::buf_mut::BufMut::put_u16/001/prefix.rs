// Answer 0

#[test]
fn test_put_u16_min_value() {
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

        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.data.len()..self.data.len() + remaining]
        }
    }

    let mut buf = TestBuf::new(2);
    buf.put_u16(0);
}

#[test]
fn test_put_u16_max_value() {
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

        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.data.len()..self.data.len() + remaining]
        }
    }

    let mut buf = TestBuf::new(2);
    buf.put_u16(65535);
}

#[test]
#[should_panic]
fn test_put_u16_insufficient_capacity() {
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

        fn chunk_mut(&mut self) -> &mut [u8] {
            let remaining = self.remaining_mut();
            &mut self.data[self.data.len()..self.data.len() + remaining]
        }
    }

    let mut buf = TestBuf::new(1);
    buf.put_u16(1);
}


// Answer 0

#[test]
fn test_put_u32_le_with_exact_capacity() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.position..]
        }
    }

    let mut buf = TestBuf::new(4);
    buf.put_u32_le(0x0809A0A1);
}

#[test]
#[should_panic]
fn test_put_u32_le_with_insufficient_capacity_three() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.position..]
        }
    }

    let mut buf = TestBuf::new(3);
    buf.put_u32_le(0x0809A0A1);
}

#[test]
#[should_panic]
fn test_put_u32_le_with_no_capacity() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.position..]
        }
    }

    let mut buf = TestBuf::new(0);
    buf.put_u32_le(0x0809A0A1);
}


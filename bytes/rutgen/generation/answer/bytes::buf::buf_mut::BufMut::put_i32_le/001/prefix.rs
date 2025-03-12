// Answer 0

#[test]
fn test_put_i32_le_valid_positive() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.data[self.position..self.position + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 8],
        position: 0,
    };
    buf.put_slice(&(0x0809A0A1i32.to_le_bytes()));
}

#[test]
fn test_put_i32_le_valid_negative() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.data[self.position..self.position + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 8],
        position: 0,
    };
    buf.put_slice(&(-0x0809A0A1i32.to_le_bytes()));
}

#[test]
#[should_panic]
fn test_put_i32_le_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.data[self.position..self.position + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 3],
        position: 0,
    };
    buf.put_slice(&(0x0809A0A1i32.to_le_bytes()));
}

#[test]
fn test_put_i32_le_boundary_case() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn put_slice(&mut self, src: &[u8]) {
            assert!(self.remaining_mut() >= src.len());
            let dst = &mut self.data[self.position..self.position + src.len()];
            dst.copy_from_slice(src);
            unsafe { self.advance_mut(src.len()) };
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 4],
        position: 0,
    };
    buf.put_slice(&(0x7FFFFFFFi32.to_le_bytes()));
}


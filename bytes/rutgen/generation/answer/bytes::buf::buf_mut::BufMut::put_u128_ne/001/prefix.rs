// Answer 0

#[test]
fn test_put_u128_ne_with_min_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume a simple implementation for chunk_mut for the test
            let available = self.remaining_mut();
            let chunk = &mut self.data[self.position..self.position + available];
            unsafe { UninitSlice::assume_init_mut(chunk) }
        }
        fn put<T: super::Buf>(&mut self, _src: T) {}
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
        fn put_u8(&mut self, _n: u8) {}
        fn put_i8(&mut self, _n: i8) {}
        fn put_u16(&mut self, _n: u16) {}
        fn put_u32(&mut self, _n: u32) {}
        fn put_u64(&mut self, _n: u64) {}
        fn put_u128(&mut self, _n: u128) {}
        fn put_i128(&mut self, _n: i128) {}
        // Other methods similarly implemented as no-ops...
    }

    let mut buf = TestBuf {
        data: vec![0; 16],
        position: 0,
    };
    buf.put_u128_ne(0x01020304050607080910111213141516);
}

#[test]
fn test_put_u128_ne_with_sufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let available = self.remaining_mut();
            let chunk = &mut self.data[self.position..self.position + available];
            unsafe { UninitSlice::assume_init_mut(chunk) }
        }
        fn put<T: super::Buf>(&mut self, _src: T) {}
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
        fn put_u8(&mut self, _n: u8) {}
        fn put_i8(&mut self, _n: i8) {}
        fn put_u16(&mut self, _n: u16) {}
        fn put_u32(&mut self, _n: u32) {}
        fn put_u64(&mut self, _n: u64) {}
        fn put_u128(&mut self, _n: u128) {}
        fn put_i128(&mut self, _n: i128) {}
        // Other methods similarly implemented as no-ops...
    }

    let mut buf = TestBuf {
        data: vec![0; 32],
        position: 0,
    };
    buf.put_u128_ne(0x01020304050607080910111213141516);
}

#[test]
#[should_panic]
fn test_put_u128_ne_with_insufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let available = self.remaining_mut();
            let chunk = &mut self.data[self.position..self.position + available];
            unsafe { UninitSlice::assume_init_mut(chunk) }
        }
        fn put<T: super::Buf>(&mut self, _src: T) {}
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
        fn put_u8(&mut self, _n: u8) {}
        fn put_i8(&mut self, _n: i8) {}
        fn put_u16(&mut self, _n: u16) {}
        fn put_u32(&mut self, _n: u32) {}
        fn put_u64(&mut self, _n: u64) {}
        fn put_u128(&mut self, _n: u128) {}
        fn put_i128(&mut self, _n: i128) {}
        // Other methods similarly implemented as no-ops...
    }

    let mut buf = TestBuf {
        data: vec![0; 15],
        position: 0,
    };
    buf.put_u128_ne(0x01020304050607080910111213141516);
}


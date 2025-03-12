// Answer 0

#[test]
fn test_put_i16_ne_less_than_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let len = self.remaining_mut();
            &mut self.buffer[self.position..self.position + len] as *mut _ as *mut UninitSlice
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            unsafe { self.chunk_mut().copy_from_slice(&src[..cnt]) };
            unsafe { self.advance_mut(cnt) };
        }

        fn put_i16_ne(&mut self, n: i16) {
            self.put_slice(&n.to_ne_bytes())
        }
    }

    let mut buf = TestBuf {
        buffer: vec![0; 1],
        position: 0,
    };
    buf.put_i16_ne(0x0809);
}

#[test]
fn test_put_i16_ne_exact_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let len = self.remaining_mut();
            &mut self.buffer[self.position..self.position + len] as *mut _ as *mut UninitSlice
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            unsafe { self.chunk_mut().copy_from_slice(&src[..cnt]) };
            unsafe { self.advance_mut(cnt) };
        }

        fn put_i16_ne(&mut self, n: i16) {
            self.put_slice(&n.to_ne_bytes())
        }
    }

    let mut buf = TestBuf {
        buffer: vec![0; 2],
        position: 0,
    };
    buf.put_i16_ne(0x0809);
}

#[test]
#[should_panic]
fn test_put_i16_ne_greater_than_capacity() {
    struct TestBuf {
        buffer: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.buffer.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let len = self.remaining_mut();
            &mut self.buffer[self.position..self.position + len] as *mut _ as *mut UninitSlice
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let cnt = usize::min(src.len(), self.remaining_mut());
            unsafe { self.chunk_mut().copy_from_slice(&src[..cnt]) };
            unsafe { self.advance_mut(cnt) };
        }

        fn put_i16_ne(&mut self, n: i16) {
            self.put_slice(&n.to_ne_bytes())
        }
    }

    let mut buf = TestBuf {
        buffer: vec![0; 1],
        position: 0,
    };
    buf.put_i16_ne(0x0809);
}


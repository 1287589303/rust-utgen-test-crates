// Answer 0

#[test]
fn test_put_i32_ne_with_sufficient_capacity() {
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
            // Safety: Assuming size is as expected.
            unsafe { &mut *(self.buffer[self.position..self.position + len].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            // Implementing as in the provided context.
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let mut remaining = src;
            while !remaining.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(remaining.len(), dst.len());
                dst[..cnt].copy_from_slice(&remaining[..cnt]);
                remaining = &remaining[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }

        fn put_i32_ne(&mut self, n: i32) {
            self.put_slice(&n.to_ne_bytes());
        }
    }

    let mut buf = TestBuf { buffer: vec![0; 4], position: 0 };
    buf.put_i32_ne(0x0809A0A1);
}

#[test]
fn test_put_i32_ne_with_min_and_max_values() {
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
            // Safety: Assuming size is as expected.
            unsafe { &mut *(self.buffer[self.position..self.position + len].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let mut remaining = src;
            while !remaining.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(remaining.len(), dst.len());
                dst[..cnt].copy_from_slice(&remaining[..cnt]);
                remaining = &remaining[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }

        fn put_i32_ne(&mut self, n: i32) {
            self.put_slice(&n.to_ne_bytes());
        }
    }

    let mut buf = TestBuf { buffer: vec![0; 4], position: 0 };
    buf.put_i32_ne(-2147483648);
    
    let mut buf_max = TestBuf { buffer: vec![0; 4], position: 0 };
    buf_max.put_i32_ne(2147483647);
}

#[test]
#[should_panic]
fn test_put_i32_ne_with_insufficient_capacity() {
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
            unsafe { &mut *(self.buffer[self.position..self.position + len].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic_advance(&TryGetError {
                    requested: src.len(),
                    available: self.remaining_mut(),
                });
            }
            let mut remaining = src;
            while !remaining.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(remaining.len(), dst.len());
                dst[..cnt].copy_from_slice(&remaining[..cnt]);
                remaining = &remaining[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }

        fn put_i32_ne(&mut self, n: i32) {
            self.put_slice(&n.to_ne_bytes());
        }
    }

    let mut buf = TestBuf { buffer: vec![0; 2], position: 0 }; // Insufficient capacity
    buf.put_i32_ne(0x0809A0A1);
}


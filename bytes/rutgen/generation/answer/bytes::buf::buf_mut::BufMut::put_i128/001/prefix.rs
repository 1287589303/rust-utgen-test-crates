// Answer 0

#[test]
fn test_put_i128_positive() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice is correctly represented as a slice
            let len = self.remaining_mut();
            unsafe { &mut *(self.data[self.pos..self.data.len()].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let mut src = src;
            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buf = TestBuf { data: vec![0; 16], pos: 0 };
    buf.put_slice(&0x01020304050607080910111213141516.to_be_bytes());
}

#[test]
fn test_put_i128_negative() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice is correctly represented as a slice
            let len = self.remaining_mut();
            unsafe { &mut *(self.data[self.pos..self.data.len()].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let mut src = src;
            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buf = TestBuf { data: vec![0; 16], pos: 0 };
    buf.put_slice(&(-0x01020304050607080910111213141516).to_be_bytes());
}

#[test]
fn test_put_i128_edge_case() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Assume UninitSlice is correctly represented as a slice
            let len = self.remaining_mut();
            unsafe { &mut *(self.data[self.pos..self.data.len()].as_mut_ptr() as *mut UninitSlice) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!();
            }
            let mut src = src;
            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buf = TestBuf { data: vec![0; 16], pos: 0 };
    buf.put_slice(&(i128::MAX).to_be_bytes());
} 

#[test]
fn test_put_i128_panic_on_underflow() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {
            panic!("Should never reach here");
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            panic!("Should never reach here");
        }

        fn put_slice(&mut self, _src: &[u8]) {
            panic!();
        }
    }

    let mut buf = TestBuf { data: vec![0; 15], pos: 0 };
    let result = std::panic::catch_unwind(|| {
        buf.put_slice(&(i128::MAX).to_be_bytes());
    });
    assert!(result.is_err());
}


// Answer 0

#[test]
fn test_advance_mut_with_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }
    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Dummy implementation for the sake of the test
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let limit_value = 5;
    let mut buffer = TestBufMut {
        data: vec![0; limit_value],
        position: 0,
    };
    
    let mut limited_buffer = Limit {
        inner: buffer,
        limit: limit_value,
    };

    let cnt = limited_buffer.limit; 
    unsafe {
        limited_buffer.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_with_bound_check() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }
    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Dummy implementation for the sake of the test
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let limit_value = 1;
    let mut buffer = TestBufMut {
        data: vec![0; limit_value],
        position: 0,
    };
    
    let mut limited_buffer = Limit {
        inner: buffer,
        limit: limit_value,
    };

    let cnt = limited_buffer.limit; 
    unsafe {
        limited_buffer.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_with_large_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }
    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Dummy implementation for the sake of the test
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let limit_value = 100;
    let mut buffer = TestBufMut {
        data: vec![0; limit_value],
        position: 0,
    };
    
    let mut limited_buffer = Limit {
        inner: buffer,
        limit: limit_value,
    };

    let cnt = limited_buffer.limit; 
    unsafe {
        limited_buffer.advance_mut(cnt);
    }
}


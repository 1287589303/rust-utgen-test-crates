// Answer 0

#[test]
fn test_put_i128_ne_with_positive_integer() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl SimpleBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit_slice(&mut self.data[self.pos..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough capacity");
            }
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) }
        }
    }

    let mut buf = SimpleBuf::new(16);
    buf.put_i128_ne(0x01020304050607080910111213141516);
}

#[test]
fn test_put_i128_ne_with_negative_integer() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl SimpleBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit_slice(&mut self.data[self.pos..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough capacity");
            }
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) }
        }
    }

    let mut buf = SimpleBuf::new(16);
    buf.put_i128_ne(-0x01020304050607080910111213141516);
}

#[test]
#[should_panic]
fn test_put_i128_ne_with_insufficient_capacity() {
    struct SimpleBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl SimpleBuf {
        fn new(capacity: usize) -> Self {
            Self {
                data: vec![0; capacity],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice::new_uninit_slice(&mut self.data[self.pos..])
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Not enough capacity");
            }
            let cnt = src.len();
            self.data[self.pos..self.pos + cnt].copy_from_slice(src);
            unsafe { self.advance_mut(cnt) }
        }
    }

    let mut buf = SimpleBuf::new(15);
    buf.put_i128_ne(0x01020304050607080910111213141516);
}


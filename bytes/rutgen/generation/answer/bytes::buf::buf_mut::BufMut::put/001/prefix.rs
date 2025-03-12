// Answer 0

#[test]
#[should_panic]
fn test_put_panic_case_1() {
    struct DummyBufMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for DummyBufMut {
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
            let len = self.remaining_mut();
            unsafe { UninitSlice::from_raw_parts_mut(self.data.as_mut_ptr().add(self.position), len) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(vec![val; cnt]);
        }

        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }

        // Other required methods can be implemented as no-ops
        fn put<T: super::Buf>(&mut self, _: T) {}
    }

    let mut buf = DummyBufMut {
        data: vec![0; 5],
        position: 5,
    };

    let src = vec![1, 2, 3, 4, 5, 6]; // src.remaining() = 6, buf.remaining_mut() = 0
    buf.put(src);
}

#[test]
#[should_panic]
fn test_put_panic_case_2() {
    struct DummyBufMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for DummyBufMut {
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
            let len = self.remaining_mut();
            unsafe { UninitSlice::from_raw_parts_mut(self.data.as_mut_ptr().add(self.position), len) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(vec![val; cnt]);
        }

        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }

        // Other required methods can be implemented as no-ops
        fn put<T: super::Buf>(&mut self, _: T) {}
    }

    let mut buf = DummyBufMut {
        data: vec![0; 10],
        position: 10,
    };

    let src = vec![1; 11]; // src.remaining() = 11, buf.remaining_mut() = 0
    buf.put(src);
}

#[test]
#[should_panic]
fn test_put_panic_large_src() {
    struct DummyBufMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for DummyBufMut {
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
            let len = self.remaining_mut();
            unsafe { UninitSlice::from_raw_parts_mut(self.data.as_mut_ptr().add(self.position), len) }
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(vec![val; cnt]);
        }

        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }

        // Other required methods can be implemented as no-ops
        fn put<T: super::Buf>(&mut self, _: T) {}
    }

    let mut buf = DummyBufMut {
        data: vec![0; 15],
        position: 15,
    };

    let src = vec![1; 20]; // src.remaining() = 20, buf.remaining_mut() = 0
    buf.put(src);
}


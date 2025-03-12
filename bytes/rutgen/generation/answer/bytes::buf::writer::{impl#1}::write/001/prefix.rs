// Answer 0

#[test]
fn test_write_with_zero_length_src() {
    struct TestBuf {
        remaining: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, _: &[u8]) {}
    }

    let mut buf = TestBuf { remaining: 0 };
    let mut writer = Writer { buf: buf };
    let result = writer.write(&[]);
}

#[test]
fn test_write_with_underlying_buffer_zero_remaining() {
    struct TestBuf {
        remaining: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, _: &[u8]) {}
    }

    let mut buf = TestBuf { remaining: 0 };
    let mut writer = Writer { buf: buf };
    let result = writer.write(&[1, 2, 3]);
}

#[test]
fn test_write_with_exact_fill() {
    struct TestBuf {
        remaining: usize,
        content: Vec<u8>,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.content.extend_from_slice(src);
            self.remaining = self.remaining.saturating_sub(src.len());
        }
    }

    let mut buf = TestBuf { remaining: 3, content: Vec::new() };
    let mut writer = Writer { buf: buf };
    let result = writer.write(&[1, 2, 3]);
}

#[test]
fn test_write_with_partial_fill() {
    struct TestBuf {
        remaining: usize,
        content: Vec<u8>,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.content.extend_from_slice(src);
            self.remaining = self.remaining.saturating_sub(src.len());
        }
    }

    let mut buf = TestBuf { remaining: 5, content: Vec::new() };
    let mut writer = Writer { buf: buf };
    let result = writer.write(&[1, 2, 3]);
}

#[test]
fn test_write_with_exceeding_fill() {
    struct TestBuf {
        remaining: usize,
        content: Vec<u8>,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.content.extend_from_slice(src);
            self.remaining = self.remaining.saturating_sub(src.len());
        }
    }

    let mut buf = TestBuf { remaining: 2, content: Vec::new() };
    let mut writer = Writer { buf: buf };
    let result = writer.write(&[1, 2, 3, 4]);
}


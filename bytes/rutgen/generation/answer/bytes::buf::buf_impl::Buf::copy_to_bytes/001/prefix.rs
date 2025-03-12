// Answer 0

#[test]
fn test_copy_to_bytes_panic_case_1() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() as usize
        }

        fn take(self, limit: usize) -> Vec<u8> {
            self.data.into_iter().take(limit).collect()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            if self.remaining() < len {
                panic_advance(&TryGetError {
                    requested: len,
                    available: self.remaining(),
                });
            }

            let mut ret = crate::BytesMut::with_capacity(len);
            ret.put(self.take(len));
            ret.freeze()
        }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3] };
    let _ = buf.copy_to_bytes(5);
}

#[test]
fn test_copy_to_bytes_panic_case_2() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() as usize
        }

        fn take(self, limit: usize) -> Vec<u8> {
            self.data.into_iter().take(limit).collect()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            if self.remaining() < len {
                panic_advance(&TryGetError {
                    requested: len,
                    available: self.remaining(),
                });
            }

            let mut ret = crate::BytesMut::with_capacity(len);
            ret.put(self.take(len));
            ret.freeze()
        }
    }

    let mut buf = TestBuf { data: vec![1, 2] };
    let _ = buf.copy_to_bytes(3);
}

#[test]
fn test_copy_to_bytes_panic_case_3() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() as usize
        }

        fn take(self, limit: usize) -> Vec<u8> {
            self.data.into_iter().take(limit).collect()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            if self.remaining() < len {
                panic_advance(&TryGetError {
                    requested: len,
                    available: self.remaining(),
                });
            }

            let mut ret = crate::BytesMut::with_capacity(len);
            ret.put(self.take(len));
            ret.freeze()
        }
    }

    let mut buf = TestBuf { data: vec![] };
    let _ = buf.copy_to_bytes(1);
}


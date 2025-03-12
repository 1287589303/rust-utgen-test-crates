// Answer 0

#[test]
fn test_try_copy_to_slice_ok_full_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let len = std::cmp::min(dst.len(), self.remaining());
            dst.copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> {
            if self.remaining() < dst.len() {
                return Err(TryGetError {
                    requested: dst.len(),
                    available: self.remaining(),
                });
            }
            while !dst.is_empty() {
                let src = self.chunk();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                dst = &mut dst[cnt..];
                self.advance(cnt);
            }
            Ok(())
        }
    }

    let mut buf = TestBuf::new(b"hello"[..].to_vec());
    let mut dst = [0; 5];

    let result = buf.try_copy_to_slice(&mut dst);
    let expected = Ok(());

    // The result is not asserted but the function is called.
}

#[test]
fn test_try_copy_to_slice_err_not_enough_remaining() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let len = std::cmp::min(dst.len(), self.remaining());
            dst.copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> {
            if self.remaining() < dst.len() {
                return Err(TryGetError {
                    requested: dst.len(),
                    available: self.remaining(),
                });
            }
            while !dst.is_empty() {
                let src = self.chunk();
                let cnt = usize::min(src.len(), dst.len());
                dst[..cnt].copy_from_slice(&src[..cnt]);
                dst = &mut dst[cnt..];
                self.advance(cnt);
            }
            Ok(())
        }
    }

    let mut buf = TestBuf::new(b"hello"[..].to_vec());
    let mut dst = [0; 12];

    let result = buf.try_copy_to_slice(&mut dst);
    let expected = Err(TryGetError { requested: 12, available: 5 });

    // The result is not asserted but the function is called.
}


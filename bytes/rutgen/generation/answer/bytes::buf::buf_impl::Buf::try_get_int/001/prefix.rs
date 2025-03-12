// Answer 0

#[test]
fn test_try_get_int_success_1_byte() {
    struct TestBuf<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for TestBuf<'a> {
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
            let len = usize::min(self.remaining(), dst.len());
            dst[..len].copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.advance(1);
            byte
        }

        fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.data[self.position..self.position + nbytes].iter().rev().fold(0, |acc, &b| (acc << 8) | (b as i64));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf { data: &b"\x01"[..], position: 0 };
    let result = buf.try_get_int(1);
    // Use result as needed; currently, it's just generating the call.
}

#[test]
fn test_try_get_int_success_2_bytes() {
    struct TestBuf<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for TestBuf<'a> {
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
            let len = usize::min(self.remaining(), dst.len());
            dst[..len].copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.advance(1);
            byte
        }

        fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.data[self.position..self.position + nbytes].iter().rev().fold(0, |acc, &b| (acc << 8) | (b as i64));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf { data: &b"\x01\x02"[..], position: 0 };
    let result = buf.try_get_int(2);
    // Use result as needed; currently, it's just generating the call.
}

#[test]
fn test_try_get_int_failure() {
    struct TestBuf<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for TestBuf<'a> {
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
            let len = usize::min(self.remaining(), dst.len());
            dst[..len].copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.advance(1);
            byte
        }

        fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.data[self.position..self.position + nbytes].iter().rev().fold(0, |acc, &b| (acc << 8) | (b as i64));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf { data: &b"\x01"[..], position: 0 };
    let result = buf.try_get_int(2);
    // Use result as needed; currently, it's just generating the call.
}

#[test]
#[should_panic]
fn test_try_get_int_too_many_bytes() {
    struct TestBuf<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Buf for TestBuf<'a> {
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
            let len = usize::min(self.remaining(), dst.len());
            dst[..len].copy_from_slice(&self.chunk()[..len]);
            self.advance(len);
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.advance(1);
            byte
        }

        fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            if nbytes > 8 {
                panic!("nbytes cannot be greater than 8");
            }
            if self.remaining() < nbytes {
                return Err(TryGetError { requested: nbytes, available: self.remaining() });
            }
            let value = self.data[self.position..self.position + nbytes].iter().rev().fold(0, |acc, &b| (acc << 8) | (b as i64));
            self.advance(nbytes);
            Ok(value)
        }
    }

    let mut buf = TestBuf { data: &b"\x01"[..], position: 0 };
    let result = buf.try_get_int(9);
    // Using result should cause a panic.
}


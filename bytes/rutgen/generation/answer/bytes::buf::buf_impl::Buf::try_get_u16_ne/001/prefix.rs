// Answer 0

#[test]
fn test_try_get_u16_ne_remaining_0() {
    struct BufStruct {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for BufStruct {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let value = u16::from_ne_bytes([self.data[self.position], self.data[self.position + 1]]);
            self.position += 2;
            Ok(value)
        }
    }

    let mut buf = BufStruct {
        data: vec![],
        position: 0,
    };
    let result = buf.try_get_u16_ne();
}

#[test]
fn test_try_get_u16_ne_remaining_1() {
    struct BufStruct {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for BufStruct {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let value = u16::from_ne_bytes([self.data[self.position], self.data[self.position + 1]]);
            self.position += 2;
            Ok(value)
        }
    }

    let mut buf = BufStruct {
        data: vec![0x08],
        position: 0,
    };
    let result = buf.try_get_u16_ne();
}

#[test]
fn test_try_get_u16_ne_remaining_2() {
    struct BufStruct {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for BufStruct {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> {
            if self.remaining() < 2 {
                return Err(TryGetError {
                    requested: 2,
                    available: self.remaining(),
                });
            }
            let value = u16::from_ne_bytes([self.data[self.position], self.data[self.position + 1]]);
            self.position += 2;
            Ok(value)
        }
    }

    let mut buf = BufStruct {
        data: vec![0x08, 0x09],
        position: 0,
    };
    let result = buf.try_get_u16_ne();
}


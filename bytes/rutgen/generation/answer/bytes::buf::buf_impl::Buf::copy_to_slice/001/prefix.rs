// Answer 0

#[test]
fn test_copy_to_slice_zero_length() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf = Buffer {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut dst = [0; 0];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_one_length() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf = Buffer {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut dst = [0; 1];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_equal_length() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf = Buffer {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
}

#[should_panic]
#[test]
fn test_copy_to_slice_exceed_length() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf = Buffer {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut dst = [0; 6];
    buf.copy_to_slice(&mut dst);
}

#[should_panic]
#[test]
fn test_copy_to_slice_empty_buffer() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }
    }

    let mut buf = Buffer {
        data: vec![],
        position: 0,
    };
    let mut dst = [0; 1];
    buf.copy_to_slice(&mut dst);
}


// Answer 0

#[test]
fn test_read_until_left_valid_byte() {
    struct MockLeftReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufRead for MockLeftReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            if self.pos < self.data.len() {
                Ok(&self.data[self.pos..])
            } else {
                Ok(&[])
            }
        }

        fn consume(&mut self, amt: usize) {
            self.pos += amt.min(self.data.len() - self.pos);
        }
    }

    let left_reader = MockLeftReader { data: b"hello world".to_vec(), pos: 0 };
    let mut either_input = Either::Left(left_reader);
    let byte = b'o'; // Valid byte
    let mut buf = Vec::new();
    
    let _ = either_input.read_until(byte, &mut buf);
}

#[test]
fn test_read_until_left_empty_buffer() {
    struct MockLeftReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufRead for MockLeftReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            if self.pos < self.data.len() {
                Ok(&self.data[self.pos..])
            } else {
                Ok(&[])
            }
        }

        fn consume(&mut self, amt: usize) {
            self.pos += amt.min(self.data.len() - self.pos);
        }
    }

    let left_reader = MockLeftReader { data: b"".to_vec(), pos: 0 };
    let mut either_input = Either::Left(left_reader);
    let byte = b'a'; // Valid byte
    let mut buf = Vec::new();
    
    let _ = either_input.read_until(byte, &mut buf);
}

#[test]
fn test_read_until_left_no_match() {
    struct MockLeftReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufRead for MockLeftReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            if self.pos < self.data.len() {
                Ok(&self.data[self.pos..])
            } else {
                Ok(&[])
            }
        }

        fn consume(&mut self, amt: usize) {
            self.pos += amt.min(self.data.len() - self.pos);
        }
    }

    let left_reader = MockLeftReader { data: b"hello world".to_vec(), pos: 0 };
    let mut either_input = Either::Left(left_reader);
    let byte = b'z'; // Valid byte but not in data
    let mut buf = Vec::new();
    
    let _ = either_input.read_until(byte, &mut buf);
}

#[test]
fn test_read_until_left_boundary_case() {
    struct MockLeftReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl BufRead for MockLeftReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            if self.pos < self.data.len() {
                Ok(&self.data[self.pos..])
            } else {
                Ok(&[])
            }
        }

        fn consume(&mut self, amt: usize) {
            self.pos += amt.min(self.data.len() - self.pos);
        }
    }

    let left_reader = MockLeftReader { data: b"Boundary".to_vec(), pos: 0 };
    let mut either_input = Either::Left(left_reader);
    let byte = b'y'; // Valid byte present at the end of data
    let mut buf = Vec::new();
    
    let _ = either_input.read_until(byte, &mut buf);
}


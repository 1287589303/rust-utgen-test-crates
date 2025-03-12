// Answer 0

#[test]
fn test_read_until_right_variant_empty_buffer() {
    struct MockBufRead;

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&[]) }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let buf = &mut Vec::new();
    let right_variant = Either::Right(MockBufRead);
    let _ = right_variant.read_until(42, buf);
}

#[test]
fn test_read_until_right_variant_non_empty_buffer() {
    struct MockBufRead {
        data: Vec<u8>,
        read_position: usize,
    }

    impl MockBufRead {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data,
                read_position: 0,
            }
        }
    }

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { 
            Ok(&self.data[self.read_position..]) 
        }
        fn consume(&mut self, amt: usize) {
            self.read_position += amt;
        }
        fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
            let initial_length = buf.len();
            for &b in &self.data[self.read_position..] {
                buf.push(b);
                if b == byte {
                    self.read_position += 1;
                    return Ok(buf.len() - initial_length);
                }
            }
            Ok(buf.len() - initial_length)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mock_data = vec![1, 2, 3, 255, 42, 5];
    let buf = &mut Vec::new();
    let right_variant = Either::Right(MockBufRead::new(mock_data));
    let _ = right_variant.read_until(42, buf);
}

#[test]
fn test_read_until_right_variant_byte_boundary() {
    struct MockBufRead {
        data: Vec<u8>,
        read_position: usize,
    }

    impl MockBufRead {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data,
                read_position: 0,
            }
        }
    }

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { 
            Ok(&self.data[self.read_position..]) 
        }
        fn consume(&mut self, amt: usize) {
            self.read_position += amt;
        }
        fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
            let initial_length = buf.len();
            for &b in &self.data[self.read_position..] {
                buf.push(b);
                if b == byte {
                    self.read_position += 1;
                    return Ok(buf.len() - initial_length);
                }
            }
            Ok(buf.len() - initial_length)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mock_data = vec![0, 1, 2, 3, 4, 255]; // testing 0 and 255
    let buf = &mut Vec::new();
    let right_variant = Either::Right(MockBufRead::new(mock_data));
    let _ = right_variant.read_until(0, buf);
    let _ = right_variant.read_until(255, buf);
}


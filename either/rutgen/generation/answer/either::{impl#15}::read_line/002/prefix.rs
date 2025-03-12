// Answer 0

#[test]
fn test_read_line_left() {
    struct MockBufReadLeft;
    impl BufRead for MockBufReadLeft {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(b"Line from left\n".as_ref())
        }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.push_str("Line from left\n");
            Ok(16)
        }
    }

    let mut buf = String::new();
    let left = Either::Left(MockBufReadLeft);
    let _ = left.read_line(&mut buf);
}

#[test]
fn test_read_line_right() {
    struct MockBufReadRight;
    impl BufRead for MockBufReadRight {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(b"Line from right\n".as_ref())
        }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.push_str("Line from right\n");
            Ok(17)
        }
    }

    let mut buf = String::new();
    let right = Either::Right(MockBufReadRight);
    let _ = right.read_line(&mut buf);
}

#[test]
fn test_read_line_left_empty_buf() {
    struct MockBufReadLeft;
    impl BufRead for MockBufReadLeft {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(b"Line from left\n".as_ref())
        }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.clear();
            buf.push_str("Line from left\n");
            Ok(16)
        }
    }

    let mut buf = String::new();
    let left = Either::Left(MockBufReadLeft);
    let _ = left.read_line(&mut buf);
}

#[test]
fn test_read_line_right_eof() {
    struct MockBufReadRight;
    impl BufRead for MockBufReadRight {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&b""[..]) // Simulating EOF condition
        }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            Ok(0) // No line read
        }
    }

    let mut buf = String::new();
    let right = Either::Right(MockBufReadRight);
    let _ = right.read_line(&mut buf);
}


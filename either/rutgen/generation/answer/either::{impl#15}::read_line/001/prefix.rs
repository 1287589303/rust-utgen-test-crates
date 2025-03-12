// Answer 0

#[test]
fn test_read_line_empty_right_variant() {
    struct EmptyBufRead;

    impl BufRead for EmptyBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&[]) }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> { Ok(0) }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.clear();
            Ok(0)
        }
    }

    let mut buf = String::new();
    let either = Either::Right(EmptyBufRead);
    let _result = either.read_line(&mut buf);
}

#[test]
fn test_read_line_full_line_read() {
    struct FullLineBufRead {
        data: String,
        pos: usize,
    }

    impl FullLineBufRead {
        fn new(data: &str) -> Self {
            FullLineBufRead {
                data: String::from(data),
                pos: 0,
            }
        }
    }

    impl BufRead for FullLineBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(self.data[self.pos..].as_bytes())
        }
        fn consume(&mut self, amt: usize) {
            self.pos += amt;
        }
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> { Ok(0) }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            if self.pos < self.data.len() {
                let line = &self.data[self.pos..];
                let eol = line.find('\n').unwrap_or(line.len());
                buf.push_str(&line[..eol]);
                self.pos += eol + 1;
                Ok(eol)
            } else {
                Ok(0)
            }
        }
    }

    let mut buf = String::new();
    let either = Either::Right(FullLineBufRead::new("Hello, World!\n"));
    let _result = either.read_line(&mut buf);
}

#[test]
fn test_read_line_partial_line_read() {
    struct PartialLineBufRead {
        data: String,
        pos: usize,
    }

    impl PartialLineBufRead {
        fn new(data: &str) -> Self {
            PartialLineBufRead {
                data: String::from(data),
                pos: 0,
            }
        }
    }

    impl BufRead for PartialLineBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(self.data[self.pos..].as_bytes())
        }
        fn consume(&mut self, amt: usize) {
            self.pos += amt;
        }
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> { Ok(0) }
        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            if self.pos < self.data.len() {
                let line = &self.data[self.pos..];
                let eol = line.find('\n').unwrap_or(line.len());
                buf.push_str(&line[..eol]);
                self.pos += eol; 
                Ok(eol)
            } else {
                Ok(0)
            }
        }
    }

    let mut buf = String::new();
    let either = Either::Right(PartialLineBufRead::new("Hello, "));
    let _result = either.read_line(&mut buf);
}

#[test]
#[should_panic]
fn test_read_line_invalid_type() {
    struct InvalidBufRead;

    impl BufRead for InvalidBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&[]) }
        fn consume(&mut self, _amt: usize) {}
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> { Ok(0) }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> { panic!("Invalid type") }
    }

    let mut buf = String::new();
    let either: Either<InvalidBufRead, InvalidBufRead> = Either::Right(InvalidBufRead);
    let _result = either.read_line(&mut buf);
}


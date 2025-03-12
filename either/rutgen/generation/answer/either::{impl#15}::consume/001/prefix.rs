// Answer 0

#[test]
fn test_consume_zero_amt() {
    struct MockBufRead {
        buffer: Vec<u8>,
        consumed: usize,
    }

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.buffer[self.consumed..])
        }
        fn consume(&mut self, amt: usize) {
            self.consumed += amt;
        }
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut mock = MockBufRead { buffer: vec![1, 2, 3], consumed: 0 };
    let either: Either<(), MockBufRead> = Right(mock);
    either.consume(0);
}

#[test]
fn test_consume_small_amt() {
    struct MockBufRead {
        buffer: Vec<u8>,
        consumed: usize,
    }

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.buffer[self.consumed..])
        }
        fn consume(&mut self, amt: usize) {
            self.consumed += amt;
        }
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut mock = MockBufRead { buffer: vec![1, 2, 3], consumed: 0 };
    let either: Either<(), MockBufRead> = Right(mock);
    either.consume(1);
}

#[test]
fn test_consume_large_amt() {
    struct MockBufRead {
        buffer: Vec<u8>,
        consumed: usize,
    }

    impl BufRead for MockBufRead {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.buffer[self.consumed..])
        }
        fn consume(&mut self, amt: usize) {
            self.consumed += amt;
        }
        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut mock = MockBufRead { buffer: vec![1, 2, 3], consumed: 0 };
    let either: Either<(), MockBufRead> = Right(mock);
    either.consume(usize::MAX);
}


// Answer 0

#[test]
fn test_consume_left_zero() {
    struct LeftBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufRead for LeftBuf {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.data[self.position..])
        }

        fn consume(&mut self, amt: usize) {
            self.position += amt;
        }

        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }

        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut left_buf = LeftBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let mut either = Either::Left(left_buf);
    either.consume(0);
}

#[test]
fn test_consume_left_partial() {
    struct LeftBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufRead for LeftBuf {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.data[self.position..])
        }

        fn consume(&mut self, amt: usize) {
            self.position += amt;
        }

        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }

        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut left_buf = LeftBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let mut either = Either::Left(left_buf);
    either.consume(3);
}

#[test]
fn test_consume_left_full() {
    struct LeftBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufRead for LeftBuf {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.data[self.position..])
        }

        fn consume(&mut self, amt: usize) {
            self.position += amt;
        }

        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }

        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut left_buf = LeftBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let mut either = Either::Left(left_buf);
    either.consume(5);
} 

#[test]
fn test_consume_left_exceed() {
    struct LeftBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufRead for LeftBuf {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(&self.data[self.position..])
        }

        fn consume(&mut self, amt: usize) {
            self.position += amt;
        }

        fn read_until(&mut self, _byte: u8, _buf: &mut Vec<u8>) -> io::Result<usize> {
            Ok(0)
        }

        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mut left_buf = LeftBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let mut either = Either::Left(left_buf);
    either.consume(10);
}


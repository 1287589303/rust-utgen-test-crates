// Answer 0

#[test]
fn test_read_to_end_left() {
    struct MockReadLeft;

    impl Read for MockReadLeft {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf.copy_from_slice(b"Hello, ");
            Ok(7)
        }
    }

    let mut buf = vec![0; 10];
    let mut either = Either::Left(MockReadLeft);
    let result = either.read_to_end(&mut buf);
}

#[test]
fn test_read_to_end_right() {
    struct MockReadRight;

    impl Read for MockReadRight {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf.copy_from_slice(b"World!");
            Ok(6)
        }
    }

    let mut buf = vec![0; 10];
    let mut either = Either::Right(MockReadRight);
    let result = either.read_to_end(&mut buf);
}


// Answer 0

#[test]
fn test_flush_left_variant() {
    struct MockWrite;

    impl Write for MockWrite {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(_buf.len())
        }
        
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut left = MockWrite;
    let e = Either::Left(left);
    let _ = e.flush();
}

#[test]
fn test_flush_right_variant() {
    struct AnotherMockWrite;

    impl Write for AnotherMockWrite {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(_buf.len())
        }
        
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut right = AnotherMockWrite;
    let e = Either::Right(right);
    let _ = e.flush();
}


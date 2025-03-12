// Answer 0

#[test]
fn test_write_fmt_left_empty() {
    struct WriteImpl;
    
    impl Write for WriteImpl {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
    }

    let mut left = Either::Left(WriteImpl);
    let fmt = format_args!("");
    let _ = left.write_fmt(fmt);
}

#[test]
fn test_write_fmt_left_partial() {
    struct WriteImpl;
    
    impl Write for WriteImpl {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
    }

    let mut left = Either::Left(WriteImpl);
    let fmt = format_args!("Hello, {}!", "world");
    let _ = left.write_fmt(fmt);
}

#[test]
fn test_write_fmt_left_full() {
    struct WriteImpl;
    
    impl Write for WriteImpl {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
    }

    let mut left = Either::Left(WriteImpl);
    let fmt = format_args!("This is a test string for formatting purposes.");
    let _ = left.write_fmt(fmt);
}

#[test]
fn test_write_fmt_left_varied_lengths() {
    struct WriteImpl;
    
    impl Write for WriteImpl {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
    }

    let mut left = Either::Left(WriteImpl);
    let fmt_short = format_args!("Hi!");
    let fmt_long = format_args!("This is an example of a longer formatted string with varying lengths.");
    
    let _ = left.write_fmt(fmt_short);
    let _ = left.write_fmt(fmt_long);
}


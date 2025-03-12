// Answer 0

#[test]
fn test_seek_right_start_zero() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(0) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::Start(0);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_right_start_large_positive() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(100) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::Start(1_000_000);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_right_end_large_negative() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(0) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::End(-1_000_000);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_right_current_zero() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(0) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::Current(0);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_right_current_large_positive() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(200) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::Current(1_000_000);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_right_current_large_negative() {
    struct MockSeek;
    impl Read for MockSeek {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    }
    impl Seek for MockSeek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { Ok(0) }
    }

    let mut either = Either::Right(MockSeek);
    let pos = SeekFrom::Current(-1_000_000);
    let _ = either.seek(pos);
}


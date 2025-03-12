// Answer 0

#[test]
fn test_seek_left_start() {
    struct LeftSeekable;
    impl Seek for LeftSeekable {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            Ok(0)
        }
    }
    
    let mut either = Either::Left(LeftSeekable);
    let pos = SeekFrom::Start(0);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_left_end() {
    struct LeftSeekable;
    impl Seek for LeftSeekable {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            Ok(100)
        }
    }
    
    let mut either = Either::Left(LeftSeekable);
    let pos = SeekFrom::End(0);
    let _ = either.seek(pos);
}

#[test]
fn test_seek_left_current() {
    struct LeftSeekable;
    impl Seek for LeftSeekable {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            Ok(50)
        }
    }
    
    let mut either = Either::Left(LeftSeekable);
    let pos = SeekFrom::Current(0);
    let _ = either.seek(pos);
}

#[test]
#[should_panic]
fn test_seek_left_fail() {
    struct LeftFailingSeekable;
    impl Seek for LeftFailingSeekable {
        fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
            Err(io::Error::new(io::ErrorKind::Other, "seek failed"))
        }
    }

    let mut either = Either::Left(LeftFailingSeekable);
    let pos = SeekFrom::Start(0);
    let _ = either.seek(pos).unwrap();
}


// Answer 0

#[test]
fn test_discard_with_some_zero() {
    let mut buffer = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: Some(0),
        raw_buffer: None,
    };
    buffer.discard();
}

#[test]
fn test_discard_with_some_max() {
    let mut buffer = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: Some(255),
        raw_buffer: None,
    };
    buffer.discard();
}

#[test]
fn test_discard_with_some_middle() {
    let mut buffer = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: Some(128),
        raw_buffer: None,
    };
    buffer.discard();
}

#[test]
fn test_discard_with_none() {
    let mut buffer = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    buffer.discard();
}


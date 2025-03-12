// Answer 0

#[test]
fn test_position_first_line_first_column() {
    struct MockRead {
        iter: LineColIterator<std::slice::Iter<'static, u8>>,
        ch: Option<u8>,
    }
    
    let data: Vec<u8> = vec![b'a', b'b', b'c', b'\n'];
    let iter = LineColIterator {
        iter: data.iter(),
        line: 1,
        col: 1,
        start_of_line: 0,
    };
    
    let mut mock_reader = MockRead { iter, ch: None };
    let position = mock_reader.position();
}

#[test]
fn test_position_second_line_first_column() {
    struct MockRead {
        iter: LineColIterator<std::slice::Iter<'static, u8>>,
        ch: Option<u8>,
    }
    
    let data: Vec<u8> = vec![b'a', b'\n', b'b', b'c'];
    let iter = LineColIterator {
        iter: data.iter(),
        line: 2,
        col: 1,
        start_of_line: 1,
    };
    
    let mut mock_reader = MockRead { iter, ch: None };
    let position = mock_reader.position();
}

#[test]
fn test_position_third_line_some_column() {
    struct MockRead {
        iter: LineColIterator<std::slice::Iter<'static, u8>>,
        ch: Option<u8>,
    }
    
    let data: Vec<u8> = vec![b'a', b'\n', b'b', b'c', b'\n', b'd', b'e'];
    let iter = LineColIterator {
        iter: data.iter(),
        line: 3,
        col: 2,
        start_of_line: 5,
    };
    
    let mut mock_reader = MockRead { iter, ch: None };
    let position = mock_reader.position();
}

#[test]
fn test_position_on_boundary_line_column() {
    struct MockRead {
        iter: LineColIterator<std::slice::Iter<'static, u8>>,
        ch: Option<u8>,
    }
    
    let data: Vec<u8> = vec![b'\n', b'a', b'b', b'c'];
    let iter = LineColIterator {
        iter: data.iter(),
        line: 1,
        col: 1,
        start_of_line: 0,
    };
    
    let mut mock_reader = MockRead { iter, ch: None };
    let position = mock_reader.position();
}


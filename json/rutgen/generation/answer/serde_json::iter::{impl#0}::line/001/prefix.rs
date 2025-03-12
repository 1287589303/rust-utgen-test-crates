// Answer 0

#[test]
fn test_line_single_line() {
    struct SimpleIterator {
        data: Vec<io::Result<u8>>,
        index: usize,
    }

    impl Iterator for SimpleIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let data = vec![Ok(b'H'), Ok(b'e'), Ok(b'l'), Ok(b'l'), Ok(b'o'), Ok(b'\n')];
    let iter = SimpleIterator { data, index: 0 };
    let lci = LineColIterator::new(iter);

    let _ = lci.line();
}

#[test]
fn test_line_multiple_lines() {
    struct SimpleIterator {
        data: Vec<io::Result<u8>>,
        index: usize,
    }

    impl Iterator for SimpleIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let data = vec![Ok(b'L'), Ok(b'i'), Ok(b'n'), Ok(b'e'), Ok(b'1'), Ok(b'\n'),
                    Ok(b'L'), Ok(b'i'), Ok(b'n'), Ok(b'e'), Ok(b'2'), Ok(b'\n')];
    let iter = SimpleIterator { data, index: 0 };
    let lci = LineColIterator::new(iter);

    let _ = lci.line();
}

#[test]
fn test_line_empty_lines() {
    struct SimpleIterator {
        data: Vec<io::Result<u8>>,
        index: usize,
    }

    impl Iterator for SimpleIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let data = vec![Ok(b'\n'), Ok(b'\n'), Ok(b'L'), Ok(b'i'), Ok(b'n'), Ok(b'e'), Ok(b'3'), Ok(b'\n')];
    let iter = SimpleIterator { data, index: 0 };
    let lci = LineColIterator::new(iter);

    let _ = lci.line();
}

#[test]
fn test_line_special_characters() {
    struct SimpleIterator {
        data: Vec<io::Result<u8>>,
        index: usize,
    }

    impl Iterator for SimpleIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let data = vec![Ok(b'#'), Ok(b'@'), Ok(b'&'), Ok(b'\n'), 
                    Ok(b'F'), Ok(b'o'), Ok(b'o'), Ok(b' '), Ok(b'B'), Ok(b'a'), Ok(b'r'), Ok(b'\n')];
    let iter = SimpleIterator { data, index: 0 };
    let lci = LineColIterator::new(iter);

    let _ = lci.line();
}


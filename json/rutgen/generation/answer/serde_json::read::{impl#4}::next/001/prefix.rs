// Answer 0

#[test]
fn test_next_with_some_char() {
    struct TestIterator {
        input: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(input: Vec<u8>) -> Self {
            TestIterator { input, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Some(Ok(ch))
            } else {
                None
            }
        }
    }

    #[cfg(feature = "std")]
    {
        use crate::io::Cursor;

        let mut buffer = vec![b'a'];
        let cursor = Cursor::new(buffer);
        let iter = TestIterator::new(vec![b'b', b'c']);

        let mut io_read = IoRead {
            iter: LineColIterator { iter, line: 1, col: 1, start_of_line: 0 },
            ch: Some(b'a'),
            raw_buffer: None,
        };

        let result = io_read.next();
        // result should be Ok(Some(b'a'))
    }
}

#[test]
fn test_next_with_some_char_in_raw_buffer() {
    struct TestIterator {
        input: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(input: Vec<u8>) -> Self {
            TestIterator { input, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Some(Ok(ch))
            } else {
                None
            }
        }
    }

    #[cfg(feature = "std")]
    {
        use crate::io::Cursor;

        let mut buffer = vec![b'a'];
        let cursor = Cursor::new(buffer);
        let iter = TestIterator::new(vec![b'd', b'e']);

        let mut io_read = IoRead {
            iter: LineColIterator { iter, line: 1, col: 1, start_of_line: 0 },
            ch: Some(b'a'),
            raw_buffer: Some(vec![]),
        };

        let result = io_read.next();
        // result should be Ok(Some(b'a')) and raw_buffer should contain b'a'
    }
}


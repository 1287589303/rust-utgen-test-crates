// Answer 0

#[test]
fn test_backslash_x_nonzero_valid_chars() {
    struct CharIterator {
        data: Vec<(usize, char)>,
        index: usize,
    }

    impl Iterator for CharIterator {
        type Item = (usize, char);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let input = CharIterator {
        data: vec![(0, '1'), (1, '2')],
        index: 0,
    };

    let _ = backslash_x_nonzero(&mut input);
}

#[test]
fn test_backslash_x_nonzero_second_non_zero() {
    struct CharIterator {
        data: Vec<(usize, char)>,
        index: usize,
    }

    impl Iterator for CharIterator {
        type Item = (usize, char);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let input = CharIterator {
        data: vec![(0, 'f'), (1, '0')],
        index: 0,
    };

    let _ = backslash_x_nonzero(&mut input);
}

#[test]
fn test_backslash_x_nonzero_mixed_case() {
    struct CharIterator {
        data: Vec<(usize, char)>,
        index: usize,
    }

    impl Iterator for CharIterator {
        type Item = (usize, char);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let input = CharIterator {
        data: vec![(0, 'A'), (1, 'b')],
        index: 0,
    };

    let _ = backslash_x_nonzero(&mut input);
}


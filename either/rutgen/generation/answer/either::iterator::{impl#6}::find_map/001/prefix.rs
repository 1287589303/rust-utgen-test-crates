// Answer 0

#[test]
fn test_find_map_with_right_iterator() {
    struct RightIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_inner = RightIterator {
        values: vec![1, 2, 3],
        index: 0,
    };

    let iter_either = IterEither {
        inner: Either::Right(right_inner),
    };

    let mut iter = iter_either;

    let result = iter.find_map(|item| {
        match item {
            Either::Right(value) if value % 2 == 0 => Some(value * 2),
            _ => None,
        }
    });

    let _ = result; // Using the result for potential further assertions
}

#[test]
fn test_find_map_with_empty_right_iterator() {
    struct EmptyRightIterator {
        index: usize,
    }

    impl Iterator for EmptyRightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_inner = EmptyRightIterator { index: 0 };

    let iter_either = IterEither {
        inner: Either::Right(right_inner),
    };

    let mut iter = iter_either;

    let result = iter.find_map(|item| {
        match item {
            Either::Right(_) => Some(42), // This should never be hit as the iterator is empty
            _ => None,
        }
    });

    let _ = result; // Using the result for potential further assertions
}

#[test]
fn test_find_map_with_mixed_conditions() {
    struct MixedRightIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for MixedRightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_inner = MixedRightIterator {
        values: vec![1, 4, 5],
        index: 0,
    };

    let iter_either = IterEither {
        inner: Either::Right(right_inner),
    };

    let mut iter = iter_either;

    let result = iter.find_map(|item| {
        match item {
            Either::Right(value) if value % 2 == 0 => Some("Even"),
            _ => None,
        }
    });

    let _ = result; // Using the result for potential further assertions
}


// Answer 0

#[test]
fn test_any_with_right_iterator() {
    struct RightIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { count: 0, limit: 5 };
    let mut iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let result = iter_either.any(|x| x % 2 == 0);
}

#[test]
fn test_any_with_right_iterator_all_false() {
    struct RightIteratorAllFalse {
        count: usize,
        limit: usize,
    }

    impl Iterator for RightIteratorAllFalse {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let right_iter = RightIteratorAllFalse { count: 0, limit: 5 };
    let mut iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let result = iter_either.any(|x| x > 5);
}

#[test]
fn test_any_with_right_iterator_empty() {
    struct EmptyRightIterator;

    impl Iterator for EmptyRightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = EmptyRightIterator;
    let mut iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let result = iter_either.any(|x| x == 1);
}


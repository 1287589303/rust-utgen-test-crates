// Answer 0

#[test]
fn test_last_with_non_empty_right_iterator_returning_none() {
    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count)
            } else {
                None
            }
        }

        fn last(self) -> Option<Self::Item> {
            None
        }
    }

    struct EmptyLeftIter;

    impl Iterator for EmptyLeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = RightIter { count: 1 };
    let left_iter = EmptyLeftIter;
    let inner = Either::Right(right_iter);
    let iter_either = IterEither { inner };

    let _result = iter_either.last();
}

#[test]
fn test_last_with_non_empty_right_iterator_multiple_elements_returning_none() {
    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count)
            } else {
                None
            }
        }

        fn last(self) -> Option<Self::Item> {
            None
        }
    }

    struct EmptyLeftIter;

    impl Iterator for EmptyLeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = RightIter { count: 3 };
    let left_iter = EmptyLeftIter;
    let inner = Either::Right(right_iter);
    let iter_either = IterEither { inner };

    let _result = iter_either.last();
}


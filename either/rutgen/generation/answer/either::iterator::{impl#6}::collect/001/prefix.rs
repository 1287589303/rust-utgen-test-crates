// Answer 0

#[test]
fn test_collect_with_non_empty_right_iterator() {
    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(42) // producing a constant value
            } else {
                None
            }
        }
    }

    let right_iterator = RightIter { count: 5 };
    let iter = IterEither { inner: Either::Right(right_iterator) };
    let result: Vec<Either<i32, usize>> = iter.collect();
}

#[test]
fn test_collect_with_empty_right_iterator() {
    struct EmptyRightIter;

    impl Iterator for EmptyRightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iterator = EmptyRightIter;
    let iter = IterEither { inner: Either::Right(right_iterator) };
    let result: Vec<Either<i32, usize>> = iter.collect();
}

#[test]
fn test_collect_with_large_right_iterator() {
    struct LargeRightIter {
        count: usize,
    }

    impl Iterator for LargeRightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count) // producing decreasing values
            } else {
                None
            }
        }
    }

    let right_iterator = LargeRightIter { count: 1_000_000 };
    let iter = IterEither { inner: Either::Right(right_iterator) };
    let result: Vec<Either<i32, usize>> = iter.collect();
}


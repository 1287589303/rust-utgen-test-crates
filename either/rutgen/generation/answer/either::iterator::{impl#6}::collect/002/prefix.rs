// Answer 0

#[test]
fn test_collect_left_single_element() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let either_iter = IterEither { inner: Either::Left(left_iter) };
    let result: Vec<Either<usize, ()>> = either_iter.collect();
}

#[test]
fn test_collect_left_multiple_elements() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let either_iter = IterEither { inner: Either::Left(left_iter) };
    let result: Vec<Either<usize, ()>> = either_iter.collect();
}

#[test]
fn test_collect_left_no_elements() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let empty_iter = EmptyIterator;
    let either_iter = IterEither { inner: Either::Left(empty_iter) };
    let result: Vec<Either<usize, ()>> = either_iter.collect();
}

#[test]
fn test_collect_left_with_type() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(('a' as u8 + self.count as u8 - 1) as char)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let either_iter = IterEither { inner: Either::Left(left_iter) };
    let result: Vec<Either<char, ()>> = either_iter.collect();
}


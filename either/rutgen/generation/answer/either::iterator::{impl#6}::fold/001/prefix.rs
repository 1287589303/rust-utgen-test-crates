// Answer 0

#[test]
fn test_fold_with_right_iterator() {
    struct CountIter {
        count: usize,
    }

    impl Iterator for CountIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let right_iter = CountIter { count: 0 };
    let either_inner = Either::Right(right_iter);
    let iter_either = IterEither { inner: either_inner };

    let result = iter_either.fold(0, |acc, item| match item {
        Either::Right(val) => acc + val,
        _ => acc
    });
}

#[test]
fn test_fold_with_empty_right_iterator() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = EmptyIter;
    let either_inner = Either::Right(right_iter);
    let iter_either = IterEither { inner: either_inner };

    let result = iter_either.fold(10, |acc, item| match item {
        Either::Right(_) => acc + 1,
        _ => acc
    });
}

#[test]
fn test_fold_with_single_item_right_iterator() {
    struct SingleItemIter {
        count: usize,
    }

    impl Iterator for SingleItemIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let right_iter = SingleItemIter { count: 0 };
    let either_inner = Either::Right(right_iter);
    let iter_either = IterEither { inner: either_inner };

    let result = iter_either.fold(10, |acc, item| match item {
        Either::Right(val) => acc + val,
        _ => acc
    });
}


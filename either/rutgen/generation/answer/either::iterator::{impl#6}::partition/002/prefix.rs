// Answer 0

#[test]
fn test_partition_with_empty_left_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIterator;
    let right_iter = std::iter::once(1);
    let iter_either = IterEither { inner: Either::Left(left_iter) };

    let predicate = |&x: &Either<i32, i32>| match x {
        Either::Left(val) => val > 0,
        Either::Right(_) => false,
    };

    iter_either.partition(predicate);
}

#[test]
fn test_partition_with_single_item_left_iterator() {
    struct SingleItemIterator {
        item: i32,
        called: bool,
    }

    impl Iterator for SingleItemIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.called {
                None
            } else {
                self.called = true;
                Some(self.item)
            }
        }
    }

    let left_iter = SingleItemIterator { item: 5, called: false };
    let right_iter = std::iter::once(1);
    let iter_either = IterEither { inner: Either::Left(left_iter) };

    let predicate = |&x: &Either<i32, i32>| match x {
        Either::Left(val) => val > 0,
        Either::Right(_) => false,
    };

    iter_either.partition(predicate);
}

#[test]
fn test_partition_with_valid_and_invalid_items() {
    struct MixedIterator {
        count: usize,
    }

    impl Iterator for MixedIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                if self.count % 2 == 0 {
                    Some(self.count as i32) // Valid item
                } else {
                    Some(-(self.count as i32)) // Invalid item
                }
            } else {
                None
            }
        }
    }

    let left_iter = MixedIterator { count: 0 };
    let right_iter = std::iter::once(1);
    let iter_either = IterEither { inner: Either::Left(left_iter) };

    let predicate = |&x: &Either<i32, i32>| match x {
        Either::Left(val) => val > 0,
        Either::Right(_) => false,
    };

    iter_either.partition(predicate);
}


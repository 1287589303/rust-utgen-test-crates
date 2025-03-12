// Answer 0

#[test]
fn test_last_left_with_some_value() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }

        fn last(self) -> Option<Self::Item> {
            self.data.last().copied()
        }
    }

    let inner_iterator = TestIterator { data: vec![1, 2, 3], index: 0 };
    let inner = Either::Left(inner_iterator);
    let iter_either = IterEither { inner };

    let result = iter_either.last();
}

#[test]
fn test_last_left_with_single_value() {
    struct SingleValueIterator {
        value: Option<i32>,
    }

    impl Iterator for SingleValueIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.value.take()
        }

        fn last(self) -> Option<Self::Item> {
            self.value
        }
    }

    let inner_iterator = SingleValueIterator { value: Some(42) };
    let inner = Either::Left(inner_iterator);
    let iter_either = IterEither { inner };

    let result = iter_either.last();
}

#[test]
fn test_last_left_with_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn last(self) -> Option<Self::Item> {
            None
        }
    }

    let inner_iterator = EmptyIterator;
    let inner = Either::Left(inner_iterator);
    let iter_either = IterEither { inner };

    let result = iter_either.last();
}

#[test]
fn test_last_left_with_multiple_values() {
    struct MultiValueIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for MultiValueIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }

        fn last(self) -> Option<Self::Item> {
            self.data.last().copied()
        }
    }

    let inner_iterator = MultiValueIterator { data: vec![10, 20, 30], index: 0 };
    let inner = Either::Left(inner_iterator);
    let iter_either = IterEither { inner };

    let result = iter_either.last();
}


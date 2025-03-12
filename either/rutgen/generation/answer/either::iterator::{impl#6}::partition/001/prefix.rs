// Answer 0

#[test]
fn test_partition_with_right_iterator() {
    struct RightIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator {
        items: vec![1, 2, 3, 4, 5],
        index: 0,
    };

    let iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let partition_fn = |&item| item % 2 == 0;

    let (even, odd): (Vec<Either<i32, i32>>, Vec<Either<i32, i32>>) = iter_either.partition(partition_fn);
}

#[test]
fn test_partition_with_empty_right_iterator() {
    struct EmptyRightIterator {
        index: usize,
    }

    impl Iterator for EmptyRightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = EmptyRightIterator { index: 0 };

    let iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let partition_fn = |&item| item % 2 == 0;

    let (even, odd): (Vec<Either<i32, i32>>, Vec<Either<i32, i32>>) = iter_either.partition(partition_fn);
}


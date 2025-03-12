// Answer 0

#[test]
fn test_all_with_left_iterator_all_true() {
    struct TestIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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

    let left_iter = TestIterator {
        items: vec![1, 2, 3, 4, 5], // All values are positive
        index: 0,
    };

    let iter = IterEither {
        inner: Either::Left(left_iter),
    };

    let result = iter.all(|&x| x > 0);
}

#[test]
fn test_all_with_left_iterator_some_false() {
    struct TestIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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

    let left_iter = TestIterator {
        items: vec![1, -2, 3, 4, 5], // Contains a negative value
        index: 0,
    };

    let iter = IterEither {
        inner: Either::Left(left_iter),
    };

    let result = iter.all(|&x| x > 0);
}

#[test]
fn test_all_with_empty_left_iterator() {
    struct TestIterator {
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None // No items in iterator
        }
    }

    let left_iter = TestIterator { index: 0 };

    let iter = IterEither {
        inner: Either::Left(left_iter),
    };

    let result = iter.all(|&x| x > 0);
}


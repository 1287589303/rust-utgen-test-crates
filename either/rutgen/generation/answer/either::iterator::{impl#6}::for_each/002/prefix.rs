// Answer 0

#[test]
fn test_for_each_with_left_iterator() {
    struct LeftIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
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

    let left_iter = LeftIterator {
        values: vec![1, 2, 3],
        index: 0,
    };

    let iter_either = IterEither { inner: Either::Left(left_iter) };

    iter_either.for_each(|item| {
        // Dummy processing to illustrate test functionality
        let _ = item;
    });
}


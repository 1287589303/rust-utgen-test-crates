// Answer 0

#[test]
fn test_size_hint_right_non_empty_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(2)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let left_iter = LeftIter { count: 0 };
    let right_iter = RightIter { count: 5 };

    let either_iter = Either::Right(right_iter);
    let hint = either_iter.size_hint();
}

#[test]
fn test_size_hint_right_non_empty_iterator_with_different_elements() {
    struct LeftIter {
        items: Vec<usize>,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.items.pop()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len(), Some(self.items.len()))
        }
    }

    struct RightIter {
        items: Vec<usize>,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.items.pop()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len(), Some(self.items.len()))
        }
    }

    let left_iter = LeftIter { items: vec![1, 2, 3] };
    let right_iter = RightIter { items: vec![4, 5] };

    let either_iter = Either::Right(right_iter);
    let hint = either_iter.size_hint();
}

#[test]
fn test_size_hint_right_empty_left_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(2)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let left_iter = LeftIter { count: 0 };
    let right_iter = RightIter { count: 3 };

    let either_iter = Either::Right(right_iter);
    let hint = either_iter.size_hint();
}


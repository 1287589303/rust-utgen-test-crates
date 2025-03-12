// Answer 0

#[test]
fn test_nth_with_right_iterator_valid_index() {
    struct RightIterator {
        count: usize,
        max: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { count: 0, max: 5 };
    let either_instance = Either::Right(right_iter);
    let result = either_instance.nth(2);
}

#[test]
fn test_nth_with_right_iterator_boundary_index() {
    struct RightIterator {
        count: usize,
        max: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { count: 0, max: 5 };
    let either_instance = Either::Right(right_iter);
    let result = either_instance.nth(4); // Boundary case
}

#[test]
fn test_nth_with_right_iterator_out_of_bounds_index() {
    struct RightIterator {
        count: usize,
        max: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator { count: 0, max: 5 };
    let either_instance = Either::Right(right_iter);
    let result = either_instance.nth(10); // Out of bounds case
}


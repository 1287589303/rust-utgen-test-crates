// Answer 0

#[test]
fn test_find_with_left_iterator_empty() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0) // No actual elements
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result = either.find(|&x| x == 0);
}

#[test]
fn test_find_with_left_iterator_single_element() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // Single element
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 1 };
    let either = Either::Left(left_iter);

    let result = either.find(|&x| x == 1);
}

#[test]
fn test_find_with_left_iterator_multiple_elements() {
    struct LeftIter {
        count: usize,
        current: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                self.current += 1;
                Some(self.current) // Multiple elements 1, 2, 3, ...
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 3, current: 0 };
    let either = Either::Left(left_iter);

    let result = either.find(|&x| x == 2);
}

#[test]
fn test_find_with_right_iterator_empty() {
    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(0) // No actual elements
            } else {
                None
            }
        }
    }

    let right_iter = RightIter { count: 0 };
    let either = Either::Right(right_iter);

    let result = either.find(|&x| x == 0);
}

#[test]
fn test_find_with_right_iterator_single_element() {
    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // Single element
            } else {
                None
            }
        }
    }

    let right_iter = RightIter { count: 1 };
    let either = Either::Right(right_iter);

    let result = either.find(|&x| x == 1);
}

#[test]
fn test_find_with_right_iterator_multiple_elements() {
    struct RightIter {
        count: usize,
        current: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                self.current += 1;
                Some(self.current) // Multiple elements
            } else {
                None
            }
        }
    }

    let right_iter = RightIter { count: 3, current: 0 };
    let either = Either::Right(right_iter);

    let result = either.find(|&x| x == 2);
}


// Answer 0

#[test]
fn test_nth_left_variant() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
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

    struct RightIterator {
        count: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count + 10)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let right_iter = RightIterator { count: 0 };
    let mut either = Either::Left(left_iter);

    let result = either.nth(2);
    // result should correspond to the value from the LeftIterator at index 2
}

#[test]
fn test_nth_right_variant() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
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

    struct RightIterator {
        count: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count + 10)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let right_iter = RightIterator { count: 0 };
    let mut either = Either::Right(right_iter);

    let result = either.nth(2);
    // result should correspond to the value from the RightIterator at index 2
}

#[test]
fn test_nth_out_of_bounds_left_variant() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    struct RightIterator {
        count: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count + 10)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let right_iter = RightIterator { count: 0 };
    let mut either = Either::Left(left_iter);

    let result = either.nth(5);
    // result should be None since 5 is out of bounds
}

#[test]
fn test_nth_out_of_bounds_right_variant() {
    struct LeftIterator {
        count: usize,
    }

    impl Iterator for LeftIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    struct RightIterator {
        count: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count + 10)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIterator { count: 0 };
    let right_iter = RightIterator { count: 0 };
    let mut either = Either::Right(right_iter);

    let result = either.nth(5);
    // result should be None since 5 is out of bounds
}


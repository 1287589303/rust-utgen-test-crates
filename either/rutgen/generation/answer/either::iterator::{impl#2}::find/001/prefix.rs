// Answer 0

#[test]
fn test_find_with_matching_item_in_right_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count * 2)
            } else {
                None
            }
        }
    }

    let left = LeftIter { count: 0 };
    let right = RightIter { count: 0 };
    let either = Either::Right(right);

    let result = either.find(|&item| item == 4);
}

#[test]
fn test_find_with_no_matching_item_in_right_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        count: usize,
    }

    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count * 2)
            } else {
                None
            }
        }
    }

    let left = LeftIter { count: 0 };
    let right = RightIter { count: 0 };
    let either = Either::Right(right);

    let result = either.find(|&item| item == 10);
}

#[test]
fn test_find_with_empty_right_iterator() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
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

    struct EmptyRightIter;

    impl Iterator for EmptyRightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left = LeftIter { count: 0 };
    let right = EmptyRightIter;
    let either = Either::Right(right);

    let result = either.find(|&item| item == 2);
}


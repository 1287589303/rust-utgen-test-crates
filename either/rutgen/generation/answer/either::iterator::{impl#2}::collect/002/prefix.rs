// Answer 0

#[test]
fn test_collect_left_empty() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIter;
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_left_single() {
    struct SingleIter {
        count: usize,
    }

    impl Iterator for SingleIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iter = SingleIter { count: 0 };
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_right_empty() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIter;
    let right_iter = EmptyIter;
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_right_single() {
    struct SingleIter {
        count: usize,
    }

    impl Iterator for SingleIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iter = EmptyIter;
    let right_iter = SingleIter { count: 0 };
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_both_non_empty() {
    struct LeftIter {
        count: usize,
    }

    struct RightIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count * 10)
            } else {
                None
            }
        }
    }

    impl Iterator for RightIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count * 20)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 0 };
    let right_iter = RightIter { count: 0 };
    let either = Either::Left(left_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_both_empty() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIter;
    let right_iter = EmptyIter;
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}


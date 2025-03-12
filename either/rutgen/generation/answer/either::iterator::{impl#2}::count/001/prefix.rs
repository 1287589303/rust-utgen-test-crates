// Answer 0

#[test]
fn count_right_empty() {
    struct RightIter {
        count: usize,
    }
    
    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 0 {
                None
            } else {
                self.count += 1;
                Some(self.count - 1)
            }
        }
    }

    let right_iter = RightIter { count: 0 };
    let either = Either::Right(right_iter);
    let result = either.count();
}

#[test]
fn count_right_single_element() {
    struct RightIter {
        count: usize,
        yield_value: usize,
    }
    
    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                None
            } else {
                self.count += 1;
                Some(self.yield_value)
            }
        }
    }

    let right_iter = RightIter { count: 0, yield_value: 42 };
    let either = Either::Right(right_iter);
    let result = either.count();
}

#[test]
fn count_right_multiple_elements() {
    struct RightIter {
        current: usize,
    }
    
    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 10 {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_iter = RightIter { current: 0 };
    let either = Either::Right(right_iter);
    let result = either.count();
}

#[test]
fn count_right_large_number_of_elements() {
    struct RightIter {
        current: usize,
        max: usize,
    }
    
    impl Iterator for RightIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_iter = RightIter { current: 0, max: 1000 };
    let either = Either::Right(right_iter);
    let result = either.count();
}


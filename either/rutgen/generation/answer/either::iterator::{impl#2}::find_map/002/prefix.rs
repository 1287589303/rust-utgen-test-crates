// Answer 0

#[test]
fn test_find_map_left_some() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
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

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result: Option<i32> = either.find_map(|x| {
        if x % 2 == 0 {
            Some(x as i32 * 100) // Returns Some for even numbers
        } else {
            None // Returns None for odd numbers
        }
    });
}

#[test]
fn test_find_map_left_none() {
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

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result: Option<i32> = either.find_map(|x| None); // Always returns None
}

#[test]
fn test_find_map_left_first() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                self.count += 1;
                Some(1) // Only returns 1
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result: Option<i32> = either.find_map(|x| Some(x as i32 * 10)); // Should return Some(10)
}

#[test]
fn test_find_map_left_multiple() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 7 {
                self.count += 1;
                Some(self.count) // Yields values 1 to 7
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result: Option<i32> = either.find_map(|x| {
        if x == 3 {
            Some(x as i32 * 10) // Should return Some(30) when x is 3
        } else {
            None
        }
    });
}

#[test]
fn test_find_map_left_no_match() {
    struct LeftIter {
        count: usize,
    }

    impl Iterator for LeftIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 4 {
                self.count += 1;
                Some(self.count) // Yields 1 to 4
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { count: 0 };
    let either = Either::Left(left_iter);

    let result: Option<i32> = either.find_map(|x| {
        if x > 5 {
            Some(x as i32 + 10) // No values will match this condition
        } else {
            None
        }
    });
}


// Answer 0

#[test]
fn test_any_with_non_empty_left_iterator() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
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

    let left_iterator = LeftIter { values: vec![1, 2, 3], index: 0 };
    let right_iterator = RightIter { values: vec![4, 5, 6], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iterator) };

    let result = iter_either.any(|x| x == Either::Left(2));
}

#[test]
fn test_any_with_non_empty_right_iterator() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
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

    let left_iterator = LeftIter { values: vec![], index: 0 };
    let right_iterator = RightIter { values: vec![4, 5, 6], index: 0 };
    let iter_either = IterEither { inner: Either::Right(right_iterator) };

    let result = iter_either.any(|x| x == Either::Right(5));
}

#[test]
fn test_any_with_empty_iterators() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
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

    let left_iterator = LeftIter { values: vec![], index: 0 };
    let right_iterator = RightIter { values: vec![], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iterator) };

    let result = iter_either.any(|x| x == Either::Left(1));
}

#[test]
fn test_any_with_single_element_left_iterator() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
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

    let left_iterator = LeftIter { values: vec![3], index: 0 };
    let right_iterator = RightIter { values: vec![], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iterator) };

    let result = iter_either.any(|x| x == Either::Left(3));
}


// Answer 0

#[test]
fn test_find_map_with_left_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![1, 2, 3], index: 0 };
    let inner = Either::Left(left_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find_map(|either| {
        match either {
            Either::Left(value) => Some(value * 2), // Example mapping function
            Either::Right(_) => None,
        }
    });
}

#[test]
fn test_find_map_with_left_iterator_no_elements() {
    struct EmptyLeftIter {
        index: usize,
    }

    impl Iterator for EmptyLeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyLeftIter { index: 0 };
    let inner = Either::Left(left_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find_map(|_either| {
        None
    });
}

#[test]
fn test_find_map_with_left_iterator_with_non_matching_function() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![1, 2, 3], index: 0 };
    let inner = Either::Left(left_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find_map(|either| {
        match either {
            Either::Left(value) if value % 2 == 0 => Some(value * 2),
            _ => None,
        }
    });
}


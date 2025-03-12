// Answer 0

#[test]
fn test_position_with_left_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIter { data: vec![1, 2, 3, 4, 5], index: 0 };
    let either = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either };

    let result = iter_either.position(|item| {
        if let Either::Left(val) = item {
            *val == 3
        } else {
            false
        }
    });
}

#[test]
fn test_position_with_no_match() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIter { data: vec![1, 2, 3, 4, 5], index: 0 };
    let either = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either };

    let result = iter_either.position(|item| {
        if let Either::Left(val) = item {
            *val == 6  // No element matches
        } else {
            false
        }
    });
}

#[test]
fn test_position_with_empty_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIter { data: vec![], index: 0 };
    let either = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either };

    let result = iter_either.position(|_item| true); // Always returns None
}


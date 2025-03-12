// Answer 0

#[test]
fn test_nth_left_iterator_valid_index() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[n])
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIterator { data: vec![10, 20, 30], index: 0 };
    let either_inner = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either_inner };

    let result = iter_either.nth(1); // valid index within bounds
}

#[test]
fn test_nth_left_iterator_first_element() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[n])
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIterator { data: vec![10, 20, 30], index: 0 };
    let either_inner = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either_inner };

    let result = iter_either.nth(0); // first element
}

#[test]
fn test_nth_left_iterator_last_element() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIterator {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[n])
            } else {
                None
            }
        }
    }

    let left_iterator = LeftIterator { data: vec![10, 20, 30], index: 0 };
    let either_inner = Either::Left(left_iterator);
    let mut iter_either = IterEither { inner: either_inner };

    let result = iter_either.nth(2); // last element
}


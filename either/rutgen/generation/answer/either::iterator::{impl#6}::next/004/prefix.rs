// Answer 0

#[test]
fn test_next_with_left_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![1, 2, 3], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iter) };
    
    let mut iterator = iter_either;
    let result = iterator.next();
}

#[test]
fn test_next_with_empty_right_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![4], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iter) };
    
    let mut iterator = iter_either;
    let result = iterator.next();
}

#[test]
fn test_next_with_non_empty_right_iterator() {
    struct LeftIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }
    
    struct RightIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { data: vec![5], index: 0 };
    let right_iter = RightIter { data: vec![10, 20], index: 0 };
    let iter_either = IterEither { inner: Either::Left(left_iter) };
    
    let mut iterator = iter_either;
    let result = iterator.next();
}


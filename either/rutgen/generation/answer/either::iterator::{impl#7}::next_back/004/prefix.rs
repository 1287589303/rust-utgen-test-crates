// Answer 0

#[test]
fn test_next_back_with_left_iterator() {
    struct LeftIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl LeftIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: data.len() }
        }
    }

    impl DoubleEndedIterator for LeftIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let left_iter = LeftIterator::new(vec![1, 2, 3]);
    let inner = Either::Left(left_iter);
    let mut iter = IterEither { inner };

    let result = iter.next_back();
}

#[test]
fn test_next_back_with_left_iterator_boundary() {
    struct LeftIteratorBoundary {
        data: Vec<i32>,
        index: usize,
    }

    impl LeftIteratorBoundary {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: data.len() }
        }
    }

    impl DoubleEndedIterator for LeftIteratorBoundary {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let left_iter = LeftIteratorBoundary::new(vec![42]);
    let inner = Either::Left(left_iter);
    let mut iter = IterEither { inner };

    let result = iter.next_back();
} 

#[test]
fn test_next_back_with_left_iterator_empty() {
    struct LeftIteratorEmpty {
        data: Vec<i32>,
        index: usize,
    }

    impl LeftIteratorEmpty {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: data.len() }
        }
    }

    impl DoubleEndedIterator for LeftIteratorEmpty {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let left_iter = LeftIteratorEmpty::new(vec![]);
    let inner = Either::Left(left_iter);
    let mut iter = IterEither { inner };

    let result = iter.next_back();
}


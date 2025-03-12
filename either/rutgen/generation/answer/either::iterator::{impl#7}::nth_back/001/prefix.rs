// Answer 0

#[test]
fn test_nth_back_zero_none() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }
    
    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                self.index = self.data.len() - n - 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }
    }

    let iter = TestIterator { data: vec![1, 2, 3], index: 3 };
    let inner = Either::Right(iter);
    let mut iter_either = IterEither { inner };
    
    let result = iter_either.nth_back(0);
}

#[test]
fn test_nth_back_out_of_bounds_none() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }
    
    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                self.index = self.data.len() - n - 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }
    }

    let iter = TestIterator { data: vec![1, 2, 3], index: 3 };
    let inner = Either::Right(iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.nth_back(5);
}


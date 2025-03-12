// Answer 0

#[test]
fn test_len_left_empty() {
    struct EmptyIterator;

    impl ExactSizeIterator for EmptyIterator {
        type Item = ();

        fn len(&self) -> usize {
            0
        }
        
        fn is_empty(&self) -> bool {
            true
        }
        
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIterator;
    let either = Either::Left(left_iter);
    let _result = either.len();
}

#[test]
fn test_len_left_single() {
    struct SingleIterator {
        called: bool,
    }

    impl ExactSizeIterator for SingleIterator {
        type Item = i32;

        fn len(&self) -> usize {
            1
        }
        
        fn is_empty(&self) -> bool {
            false
        }

        fn next(&mut self) -> Option<Self::Item> {
            if !self.called {
                self.called = true;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iter = SingleIterator { called: false };
    let either = Either::Left(left_iter);
    let _result = either.len();
}

#[test]
fn test_len_left_multiple() {
    struct MultipleIterator {
        count: usize,
    }
    
    impl ExactSizeIterator for MultipleIterator {
        type Item = i32;

        fn len(&self) -> usize {
            self.count
        }
        
        fn is_empty(&self) -> bool {
            self.count == 0
        }

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // arbitrarily returning 1
            } else {
                None
            }
        }
    }

    let left_iter = MultipleIterator { count: 5 };
    let either = Either::Left(left_iter);
    let _result = either.len();
}

#[test]
fn test_len_right_empty() {
    struct EmptyIterator;

    impl ExactSizeIterator for EmptyIterator {
        type Item = ();

        fn len(&self) -> usize {
            0
        }
        
        fn is_empty(&self) -> bool {
            true
        }
        
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    struct LeftIterator;

    impl ExactSizeIterator for LeftIterator {
        type Item = i32;

        fn len(&self) -> usize {
            0 // arbitrary value allowed for test case
        }

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = LeftIterator;
    let right_iter = EmptyIterator;
    let either = Either::Right(right_iter);
    let _result = either.len();
}

#[test]
fn test_len_right_multiple() {
    struct RightIterator {
        count: usize,
    }

    impl ExactSizeIterator for RightIterator {
        type Item = i32;

        fn len(&self) -> usize {
            self.count
        }

        fn is_empty(&self) -> bool {
            self.count == 0
        }

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // arbitrary but valid value
            } else {
                None
            }
        }
    }

    struct LeftIterator;

    impl ExactSizeIterator for LeftIterator {
        type Item = i32;

        fn len(&self) -> usize {
            3 // Making sure it’s compatible
        }

        fn next(&mut self) -> Option<Self::Item> {
            Some(1) // arbitrary but valid value
        }
    }

    let left_iter = LeftIterator;
    let right_iter = RightIterator { count: 7 };
    let either = Either::Right(right_iter);
    let _result = either.len();
}


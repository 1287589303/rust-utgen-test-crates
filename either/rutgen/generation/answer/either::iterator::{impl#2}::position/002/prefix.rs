// Answer 0

#[test]
fn test_position_empty_left_iterator() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let left_iter = EmptyIter;
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    
    let position = either.position(|_| true);
}

#[test]
fn test_position_single_element_left_iterator_true() {
    struct SingleIter {
        count: usize,
    }

    impl Iterator for SingleIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iter = SingleIter { count: 0 };
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    
    let position = either.position(|x| *x == 42);
}

#[test]
fn test_position_single_element_left_iterator_false() {
    struct SingleIter {
        count: usize,
    }

    impl Iterator for SingleIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(42)
            } else {
                None
            }
        }
    }

    let left_iter = SingleIter { count: 0 };
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    
    let position = either.position(|x| *x == 100);
}

#[test]
fn test_position_multiple_elements_left_iterator() {
    struct MultiIter {
        count: usize,
    }

    impl Iterator for MultiIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let value = self.count;
            if value < 5 {
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = MultiIter { count: 0 };
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    
    let position = either.position(|x| *x == 3);
}

#[test]
fn test_position_no_matching_element_left_iterator() {
    struct MultiIter {
        count: usize,
    }

    impl Iterator for MultiIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let value = self.count;
            if value < 5 {
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = MultiIter { count: 0 };
    let right_iter = EmptyIter;
    let either = Either::Left(left_iter);
    
    let position = either.position(|x| *x == 10);
}


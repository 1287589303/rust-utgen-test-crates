// Answer 0

#[test]
fn test_partition_right_single_element() {
    struct MyIterator {
        current: usize,
    }

    impl Iterator for MyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 1 {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }
    
    let right_iter = MyIterator { current: 0 };
    let either = Either::Right(right_iter);
    
    let predicate = |&x: &usize| x % 2 == 0;
    
    let (even, odd): (Vec<usize>, Vec<usize>) = either.partition(predicate);
}

#[test]
fn test_partition_right_multiple_elements() {
    struct MyIterator {
        current: usize,
    }

    impl Iterator for MyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 3 {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }
    
    let right_iter = MyIterator { current: 0 };
    let either = Either::Right(right_iter);
    
    let predicate = |&x: &usize| x % 2 == 0;
    
    let (even, odd): (Vec<usize>, Vec<usize>) = either.partition(predicate);
}

#[test]
fn test_partition_right_no_elements() {
    struct MyIterator {
        current: usize,
    }

    impl Iterator for MyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
    
    let right_iter = MyIterator { current: 0 };
    let either = Either::Right(right_iter);
    
    let predicate = |&x: &usize| x % 2 == 0;
    
    let (even, odd): (Vec<usize>, Vec<usize>) = either.partition(predicate);
}

#[test]
fn test_partition_right_boundary_case() {
    struct MyIterator {
        current: usize,
    }

    impl Iterator for MyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let value = self.current;
            self.current += 1;
            Some(value)
        }
    }
    
    let right_iter = MyIterator { current: 0 };
    let either = Either::Right(right_iter);
    
    let predicate = |&x: &usize| x < 2; // should partition 0 and 1 into different vectors
    
    let (less_than_two, others): (Vec<usize>, Vec<usize>) = either.partition(predicate);
}


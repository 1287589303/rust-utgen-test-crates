// Answer 0

#[test]
fn test_find_map_with_right() {
    struct RightIterator {
        values: Vec<i32>,
        index: usize,
    }
    
    impl Iterator for RightIterator {
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

    let right_iter = RightIterator { values: vec![1, 2, 3], index: 0 };
    let either = Either::Right(right_iter);
    
    let closure = |x| if x % 2 == 0 { Some(x * 2) } else { None };

    let _result = either.find_map(closure);
}

#[test]
fn test_find_map_with_left() {
    struct LeftIterator {
        values: Vec<i32>,
        index: usize,
    }
    
    impl Iterator for LeftIterator {
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

    let left_iter = LeftIterator { values: vec![4, 5, 6], index: 0 };
    let either = Either::Left(left_iter);
    
    let closure = |x| if x > 5 { Some(x * 2) } else { None };

    let _result = either.find_map(closure);
}

#[test]
fn test_find_map_with_no_matching_left() {
    struct LeftIterator {
        values: Vec<i32>,
        index: usize,
    }
    
    impl Iterator for LeftIterator {
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

    let left_iter = LeftIterator { values: vec![1, 3, 5], index: 0 };
    let either = Either::Left(left_iter);
    
    let closure = |x| if x % 2 == 0 { Some(x * 2) } else { None };

    let _result = either.find_map(closure);
}


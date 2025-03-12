// Answer 0

#[test]
fn test_position_with_matching_predicate() {
    struct RightIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator {
        items: vec![1, 2, 3, 4, 5],
        index: 0,
    };
    
    let either = Either::Right(right_iter);

    let result = either.position(|&x| x == 3);
}

#[test]
fn test_position_with_no_matching_predicate() {
    struct RightIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator {
        items: vec![1, 2, 3, 4, 5],
        index: 0,
    };
    
    let either = Either::Right(right_iter);

    let result = either.position(|&x| x == 6);
}

#[test]
fn test_position_with_empty_iterator() {
    struct RightIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator {
        items: vec![],
        index: 0,
    };
    
    let either = Either::Right(right_iter);

    let result = either.position(|&x| x == 1);
}


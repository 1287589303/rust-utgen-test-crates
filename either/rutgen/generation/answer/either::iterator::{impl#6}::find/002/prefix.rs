// Answer 0

#[test]
fn test_find_left_value() {
    struct LeftIterator {
        data: Vec<i32>,
        pos: usize,
    }
    
    impl Iterator for LeftIterator {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos < self.data.len() {
                let value = self.data[self.pos];
                self.pos += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let inner = Either::Left(LeftIterator { data: vec![1, 2, 3, 4, 5], pos: 0 });
    let mut iter = IterEither { inner };
    
    let result = iter.find(|item| matches!(item, Either::Left(value) if *value == 3));
}

#[test]
fn test_find_left_value_not_found() {
    struct LeftIterator {
        data: Vec<i32>,
        pos: usize,
    }
    
    impl Iterator for LeftIterator {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos < self.data.len() {
                let value = self.data[self.pos];
                self.pos += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let inner = Either::Left(LeftIterator { data: vec![1, 2, 3, 4, 5], pos: 0 });
    let mut iter = IterEither { inner };
    
    let result = iter.find(|item| matches!(item, Either::Left(value) if *value == 6));
}

#[test]
fn test_find_with_complex_predicate() {
    struct LeftIterator {
        data: Vec<i32>,
        pos: usize,
    }
    
    impl Iterator for LeftIterator {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos < self.data.len() {
                let value = self.data[self.pos];
                self.pos += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let inner = Either::Left(LeftIterator { data: vec![1, 2, 3, 4, 5], pos: 0 });
    let mut iter = IterEither { inner };
    
    let result = iter.find(|item| matches!(item, Either::Left(value) if value % 2 == 0));
}

#[test]
fn test_find_empty_iterator() {
    struct EmptyIterator {
        pos: usize,
    }
    
    impl Iterator for EmptyIterator {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let inner = Either::Left(EmptyIterator { pos: 0 });
    let mut iter = IterEither { inner };
    
    let result = iter.find(|_| true);
}


// Answer 0

#[test]
fn test_all_with_right_iterator_all_true() {
    struct RightIter {
        items: Vec<i32>,
        current: usize,
    }
    
    impl Iterator for RightIter {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.items.len() {
                let item = self.items[self.current];
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    let right_iter = RightIter { items: vec![2, 4, 6], current: 0 };
    let either = Either::Right(right_iter);
    
    let result = either.all(|&x| x % 2 == 0);
}

#[test]
fn test_all_with_right_iterator_all_false() {
    struct RightIter {
        items: Vec<i32>,
        current: usize,
    }
    
    impl Iterator for RightIter {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.items.len() {
                let item = self.items[self.current];
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    let right_iter = RightIter { items: vec![1, 3, 5], current: 0 };
    let either = Either::Right(right_iter);
    
    let result = either.all(|&x| x % 2 == 0);
}

#[test]
fn test_all_with_right_iterator_mixed() {
    struct RightIter {
        items: Vec<i32>,
        current: usize,
    }
    
    impl Iterator for RightIter {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.items.len() {
                let item = self.items[self.current];
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    let right_iter = RightIter { items: vec![1, 2, 3], current: 0 };
    let either = Either::Right(right_iter);
    
    let result = either.all(|&x| x % 2 == 0);
}

#[test]
fn test_all_with_empty_right_iterator() {
    struct RightIter {
        items: Vec<i32>,
        current: usize,
    }
    
    impl Iterator for RightIter {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.items.len() {
                let item = self.items[self.current];
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    let right_iter = RightIter { items: vec![], current: 0 };
    let either = Either::Right(right_iter);
    
    let result = either.all(|&x| x > 0);
}


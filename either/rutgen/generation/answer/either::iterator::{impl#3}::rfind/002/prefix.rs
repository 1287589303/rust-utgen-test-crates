// Answer 0

#[test]
fn test_rfind_left_non_empty_matching() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len() - self.index, Some(self.items.len() - self.index))
        }
    }

    impl DoubleEndedIterator for LeftIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                self.index += 1;
                Some(self.items[self.index - 1])
            } else {
                None
            }
        }

        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            let mut rev_index = self.items.len();
            while rev_index > 0 {
                rev_index -= 1;
                if predicate(&self.items[rev_index]) {
                    return Some(self.items[rev_index]);
                }
            }
            None
        }
    }

    let left_iter = LeftIter {
        items: vec![1, 2, 3, 4, 5],
        index: 0,
    };

    let either_instance = Either::Left(left_iter);
    
    let result = either_instance.rfind(|&x| x == 4);
}

#[test]
fn test_rfind_left_non_empty_matching_first() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len() - self.index, Some(self.items.len() - self.index))
        }
    }

    impl DoubleEndedIterator for LeftIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                self.index += 1;
                Some(self.items[self.index - 1])
            } else {
                None
            }
        }

        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            let mut rev_index = self.items.len();
            while rev_index > 0 {
                rev_index -= 1;
                if predicate(&self.items[rev_index]) {
                    return Some(self.items[rev_index]);
                }
            }
            None
        }
    }

    let left_iter = LeftIter {
        items: vec![5, 2, 3, 4, 1],
        index: 0,
    };

    let either_instance = Either::Left(left_iter);
    
    let result = either_instance.rfind(|&x| x == 5);
}

#[test]
fn test_rfind_left_non_empty_matching_last() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len() - self.index, Some(self.items.len() - self.index))
        }
    }

    impl DoubleEndedIterator for LeftIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                self.index += 1;
                Some(self.items[self.index - 1])
            } else {
                None
            }
        }

        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            let mut rev_index = self.items.len();
            while rev_index > 0 {
                rev_index -= 1;
                if predicate(&self.items[rev_index]) {
                    return Some(self.items[rev_index]);
                }
            }
            None
        }
    }

    let left_iter = LeftIter {
        items: vec![1, 2, 3, 4, 5],
        index: 0,
    };

    let either_instance = Either::Left(left_iter);
    
    let result = either_instance.rfind(|&x| x == 1);
}


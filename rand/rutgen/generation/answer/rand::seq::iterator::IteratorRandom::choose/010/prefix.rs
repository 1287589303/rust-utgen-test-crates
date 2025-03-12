// Answer 0

#[cfg(test)]
fn test_choose_case_lower_equals_1() {
    struct TestIterator {
        items: Vec<u32>,
        index: usize,
    }
    
    impl Iterator for TestIterator {
        type Item = u32;
        
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
            (1, Some(self.items.len()))
        }
    }
    
    struct DummyRng;
    
    impl Rng for DummyRng {
        /* Implement dummy random_range method */
    }
    
    let items = vec![1, 2];
    let iterator = TestIterator { items, index: 0 };
    let mut rng = DummyRng;
    
    let _result = iterator.choose(&mut rng);
}

#[cfg(test)]
fn test_choose_case_elem_is_none() {
    struct EmptyIterator {
        index: usize,
    }
    
    impl Iterator for EmptyIterator {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            None 
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, None)
        }
    }
    
    struct DummyRng;
    
    impl Rng for DummyRng {
        /* Implement dummy random_range method */
    }
    
    let iterator = EmptyIterator { index: 0 };
    let mut rng = DummyRng;
    
    let _result = iterator.choose(&mut rng);
}

#[cfg(test)]
fn test_choose_case_upper_is_none() {
    struct MultiItemIterator {
        items: Vec<u32>,
        index: usize,
    }
    
    impl Iterator for MultiItemIterator {
        type Item = u32;
        
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
            (3, None)
        }
    }
    
    struct DummyRng;
    
    impl Rng for DummyRng {
        /* Implement dummy random_range method */
    }
    
    let items = vec![1, 2, 3];
    let iterator = MultiItemIterator { items, index: 0 };
    let mut rng = DummyRng;
    
    let _result = iterator.choose(&mut rng);
}


// Answer 0

#[test]
fn test_choose_weighted_valid_case() {
    struct TestData<'a> {
        items: &'a [(char, u32)],
    }
    
    impl<'a> Index<usize> for TestData<'a> {
        type Output = (char, u32);
        
        fn index(&self, index: usize) -> &(char, u32) {
            &self.items[index]
        }
    }
    
    impl<'a> IndexedRandom for TestData<'a> {
        fn len(&self) -> usize {
            self.items.len()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    let choices = TestData { items: &[('a', 2), ('b', 1), ('c', 1)] };
    let mut rng = rand::thread_rng();
    
    let result = choices.choose_weighted(&mut rng, |item| item.1);
    let _ = result.unwrap(); // Ensure it compiles and runs without failure
}

#[test]
fn test_choose_weighted_empty_slice() {
    struct TestData<'a> {
        items: &'a [(char, u32)],
    }

    impl<'a> Index<usize> for TestData<'a> {
        type Output = (char, u32);
        
        fn index(&self, index: usize) -> &(char, u32) {
            &self.items[index]
        }
    }

    impl<'a> IndexedRandom for TestData<'a> {
        fn len(&self) -> usize {
            self.items.len()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    let choices = TestData { items: &[] };
    let mut rng = rand::thread_rng();
    
    let result = choices.choose_weighted(&mut rng, |item| item.1);
    let _ = result.unwrap_err(); // Ensure it compiles and runs without failure
}

#[test]
fn test_choose_weighted_with_zero_weights() {
    struct TestData<'a> {
        items: &'a [(char, i32)],
    }

    impl<'a> Index<usize> for TestData<'a> {
        type Output = (char, i32);

        fn index(&self, index: usize) -> &(char, i32) {
            &self.items[index]
        }
    }

    impl<'a> IndexedRandom for TestData<'a> {
        fn len(&self) -> usize {
            self.items.len()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    let choices = TestData { items: &[('a', 0), ('b', 0), ('c', 0)] };
    let mut rng = rand::thread_rng();
    
    let result = choices.choose_weighted(&mut rng, |item| item.1);
    let _ = result.unwrap_err(); // Ensure it compiles and runs without failure
}

#[test]
fn test_choose_weighted_valid_bounds() {
    struct TestData<'a> {
        items: &'a [(char, u32)],
    }

    impl<'a> Index<usize> for TestData<'a> {
        type Output = (char, u32);
        
        fn index(&self, index: usize) -> &(char, u32) {
            &self.items[index]
        }
    }

    impl<'a> IndexedRandom for TestData<'a> {
        fn len(&self) -> usize {
            self.items.len()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    let choices = TestData { items: &[('a', 1), ('b', 1), ('c', 1), ('d', 1)] };
    let mut rng = rand::thread_rng();
    
    let result = choices.choose_weighted(&mut rng, |item| item.1);
    let _ = result.unwrap(); // Ensure it compiles and runs without failure
}


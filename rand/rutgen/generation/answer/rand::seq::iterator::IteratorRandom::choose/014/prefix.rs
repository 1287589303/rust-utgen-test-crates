// Answer 0

#[test]
fn test_choose_with_upper_hint_false_lower_one() {
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
            (self.items.len(), Some(self.items.len()))
        }
    }

    let items = vec![1, 2, 3];
    let mut rng = rand::rngs::OsRng;
    let iter = TestIterator { items, index: 0 };
    let _result = iter.choose(&mut rng);
}

#[test]
fn test_choose_with_lower_1_and_non_none_elem() {
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
            (1, Some(2))
        }
    }

    let items = vec![5];
    let mut rng = rand::rngs::OsRng;
    let iter = TestIterator { items, index: 0 };
    let _result = iter.choose(&mut rng);
}

#[test]
fn test_choose_with_lower_greater_than_one_and_random_selection() {
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
            (3, Some(5))
        }
    }

    let items = vec![1, 2, 3];
    let mut rng = rand::rngs::OsRng;  
    let iter = TestIterator { items, index: 0 };
    let _result = iter.choose(&mut rng);
}


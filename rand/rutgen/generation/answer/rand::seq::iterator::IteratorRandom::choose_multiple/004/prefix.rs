// Answer 0

#[test]
fn test_choose_multiple_having_fewer_elements_than_amount() {
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
    }

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.len();
            range.start + self.current
        }
    }

    let iterator = TestIterator {
        items: vec![1, 2, 3],
        index: 0,
    };
    
    let mut rng = MockRng { current: 0 };
    let amount = 5; // greater than self.len()
    let result = iterator.choose_multiple(&mut rng, amount);
    // Testing input where reservoir.len() should be less than amount.
}

#[test]
fn test_choose_multiple_exceeding_iterator_length() {
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
    }

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.len();
            range.start + self.current
        }
    }

    let iterator = TestIterator {
        items: vec![10, 20], // less than amount
        index: 0,
    };

    let mut rng = MockRng { current: 0 };
    let amount = 3; // amount greater than self.len()
    let result = iterator.choose_multiple(&mut rng, amount);
    // Testing input where reservoir.len() should be less than amount due to few items.
}


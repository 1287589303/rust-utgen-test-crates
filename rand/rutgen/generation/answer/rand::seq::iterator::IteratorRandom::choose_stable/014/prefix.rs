// Answer 0

#[test]
fn test_choose_stable_with_lower_condition_false() {
    struct TestIterator {
        items: Vec<i32>,
        index: usize,
    }
    
    impl Iterator for TestIterator {
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

    struct MockRng;
    
    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            range.start // deterministic for testing
        }
    }

    let rng = &mut MockRng;
    let iterator = TestIterator {
        items: vec![1, 2],
        index: 0,
    };

    let _ = iterator.choose_stable(rng);
}

#[test]
fn test_choose_stable_with_lower_condition_true() {
    struct TestIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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
            (2, Some(2))
        }
    }

    struct MockRng;
    
    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            range.end - 1 // deterministic for testing
        }
    }

    let rng = &mut MockRng;
    let iterator = TestIterator {
        items: vec![1, 2],
        index: 0,
    };

    let _ = iterator.choose_stable(rng);
}


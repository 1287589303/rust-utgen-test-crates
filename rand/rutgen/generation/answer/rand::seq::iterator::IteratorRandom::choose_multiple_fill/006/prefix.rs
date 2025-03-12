// Answer 0

#[test]
fn test_choose_multiple_fill_exact_fit() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 5 {
                self.current += 1;
                Some(self.current)
            } else {
                None
            }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (5 - self.current, Some(5 - self.current))
        }
    }

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = MockRng { current: 0 };
    let mut buf = [0; 5];
    let iterator = TestIterator { current: 0 };
    let result = iterator.choose_multiple_fill(&mut rng, &mut buf);
}

#[test]
fn test_choose_multiple_fill_exceeding_elements() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 3 {
                self.current += 1;
                Some(self.current)
            } else {
                None
            }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (3 - self.current, Some(3 - self.current))
        }
    }

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.current = (self.current + 1) % range.end;
            self.current
        }
    }

    let mut rng = MockRng { current: 0 };
    let mut buf = [0; 5];
    let iterator = TestIterator { current: 0 };
    let result = iterator.choose_multiple_fill(&mut rng, &mut buf);
}

#[test]
fn test_choose_multiple_fill_exact_empty_iterator() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            0 // No selection, as the iterator is empty.
        }
    }

    let mut rng = MockRng { current: 0 };
    let mut buf = [0; 3];
    let iterator = TestIterator { current: 0 };
    let result = iterator.choose_multiple_fill(&mut rng, &mut buf);
}


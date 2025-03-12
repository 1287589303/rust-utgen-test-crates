// Answer 0

#[test]
fn test_choose_non_exact_even() {
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
            (self.items.len(), Some(self.items.len()))
        }
    }

    let mut rng = rand::rngs::ThreadRng::default();
    let iter = TestIterator {
        items: (0..10).collect(),
        index: 0,
    };
    let result = iter.choose(&mut rng);
}

#[test]
fn test_choose_non_exact_odd() {
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
            (self.items.len(), Some(self.items.len()))
        }
    }

    let mut rng = rand::rngs::ThreadRng::default();
    let iter = TestIterator {
        items: (0..9).collect(),
        index: 0,
    };
    let result = iter.choose(&mut rng);
}

#[test]
fn test_choose_multiple_elements() {
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
            (self.items.len(), Some(self.items.len()))
        }
    }

    let mut rng = rand::rngs::ThreadRng::default();
    let iter = TestIterator {
        items: (0..15).collect(),
        index: 0,
    };
    let result = iter.choose(&mut rng);
}


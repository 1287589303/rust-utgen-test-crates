// Answer 0

#[test]
fn test_rfind_left_with_matching_predicate() {
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
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            while self.index > 0 {
                self.index -= 1;
                if predicate(&self.items[self.index]) {
                    return Some(self.items[self.index]);
                }
            }
            None
        }
    }

    let iter = TestIterator { items: vec![1, 2, 3, 4, 5], index: 0 };
    let iter_either = IterEither { inner: Either::Left(iter) };
    let mut iter_either_mut = iter_either.clone();

    iter_either_mut.rfind(|&x| x == 3);
}

#[test]
fn test_rfind_left_with_non_matching_predicate() {
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
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            while self.index > 0 {
                self.index -= 1;
                if predicate(&self.items[self.index]) {
                    return Some(self.items[self.index]);
                }
            }
            None
        }
    }

    let iter = TestIterator { items: vec![1, 2, 3, 4, 5], index: 0 };
    let iter_either = IterEither { inner: Either::Left(iter) };
    let mut iter_either_mut = iter_either.clone();

    iter_either_mut.rfind(|&x| x == 6);
}

#[test]
fn test_rfind_left_with_empty_iterator() {
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
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
        fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            while self.index > 0 {
                self.index -= 1;
                if predicate(&self.items[self.index]) {
                    return Some(self.items[self.index]);
                }
            }
            None
        }
    }

    let iter = TestIterator { items: vec![], index: 0 };
    let iter_either = IterEither { inner: Either::Left(iter) };
    let mut iter_either_mut = iter_either.clone();

    iter_either_mut.rfind(|&x| x == 1);
}


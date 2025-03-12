// Answer 0

#[test]
fn test_choose_iterator_random_condition_1() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.data.len(), None)
        }
    }

    struct TestRng {
        calls: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            // always return the upper bound to satisfy ix < lower is false
            self.calls += 1;
            range.end - 1
        }
    }

    let mut rng = TestRng { calls: 0 };
    let iter = TestIter { data: vec![1, 2, 3], index: 0 };
    let result = iter.choose(&mut rng);
}

#[test]
fn test_choose_iterator_random_condition_2() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, None) // lower bound is 3
        }
    }

    struct TestRng {
        calls: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.calls += 1;
            // This will create the condition where random_ratio_one_over(consumed) would return true
            1 // Always select the second element
        }
    }

    let mut rng = TestRng { calls: 0 };
    let iter = TestIter { data: vec![1, 2, 3], index: 0 };
    let result = iter.choose(&mut rng);
}


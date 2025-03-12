// Answer 0

#[test]
fn test_iterator_len_hint_lo_not_equal_hi_1() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                let val = self.count;
                self.count += 1;
                Some(val)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(5))
        }
    }

    let iter = TestIterator { count: 0 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_lo_not_equal_hi_2() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                let val = self.count;
                self.count += 1;
                Some(val)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, Some(5))
        }
    }

    let iter = TestIterator { count: 0 };
    let result = iterator_len_hint(&iter);
}


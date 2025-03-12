// Answer 0

#[test]
fn test_iterator_len_hint_zero() {
    struct ZeroIterator {
        count: usize,
    }

    impl Iterator for ZeroIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(0)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let iter = ZeroIterator { count: 0 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_one() {
    struct OneIterator {
        count: usize,
    }

    impl Iterator for OneIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(0)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1))
        }
    }

    let iter = OneIterator { count: 1 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_two() {
    struct TwoIterator {
        count: usize,
    }

    impl Iterator for TwoIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(0)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, Some(2))
        }
    }

    let iter = TwoIterator { count: 2 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_large() {
    struct LargeIterator {
        count: usize,
    }

    impl Iterator for LargeIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(0)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1000, Some(1000))
        }
    }

    let iter = LargeIterator { count: 1000 };
    let result = iterator_len_hint(&iter);
}


// Answer 0

#[test]
fn test_size_hint_non_empty() {
    struct TestRawIter {
        items: usize,
    }
    
    impl TestRawIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items, Some(self.items))
        }
    }

    let mut iter = IterMut {
        inner: RawIter {
            iter: TestRawIter { items: 5 },
            items: 5,
        },
        marker: PhantomData,
    };

    let hint = iter.size_hint();
}

#[test]
fn test_size_hint_empty() {
    struct TestRawIter {
        items: usize,
    }
    
    impl TestRawIter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let mut iter = IterMut {
        inner: RawIter {
            iter: TestRawIter { items: 0 },
            items: 0,
        },
        marker: PhantomData,
    };

    let hint = iter.size_hint();
}


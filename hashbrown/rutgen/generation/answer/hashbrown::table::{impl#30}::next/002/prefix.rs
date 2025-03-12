// Answer 0

#[test]
fn test_next_returns_none_when_empty() {
    struct DummyAllocator;

    struct TestRawIterHash {
        empty: bool,
    }

    impl TestRawIterHash {
        fn new_empty() -> Self {
            Self { empty: true }
        }
        
        fn next(&mut self) -> Option<Bucket<u32>> {
            if self.empty {
                None
            } else {
                Some(Bucket { ptr: NonNull::dangling() })
            }
        }
    }

    let mut inner = TestRawIterHash::new_empty();
    let mut iter_hash = IterHash {
        inner,
        marker: PhantomData,
    };

    let result = iter_hash.next();
}


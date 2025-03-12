// Answer 0

#[test]
fn test_keys_next_some() {
    struct TestKey;
    struct TestValue;

    let key1 = TestKey;
    let key2 = TestKey;
    let value1 = TestValue;
    let value2 = TestValue;
    
    let entries = vec![(key1, value1), (key2, value2)];
    let raw_iter = RawIter::new(entries.into_iter());
    
    let keys_iter = Keys {
        inner: Iter {
            inner: raw_iter,
            marker: PhantomData,
        },
    };

    let mut keys = keys_iter;

    let result = keys.next();
}

#[test]
fn test_keys_next_none() {
    struct TestKey;
    struct TestValue;

    let entries: Vec<(TestKey, TestValue)> = Vec::new();
    let raw_iter = RawIter::new(entries.into_iter());

    let keys_iter = Keys {
        inner: Iter {
            inner: raw_iter,
            marker: PhantomData,
        },
    };

    let mut keys = keys_iter;

    let result = keys.next();
}


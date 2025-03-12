// Answer 0

#[test]
fn test_next_returns_none_when_inner_is_empty() {
    struct TestKey;
    struct TestValue;

    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 0 },
        items: 0,
    };

    let mut iter = Iter {
        inner,
        marker: PhantomData,
    };

    let result = iter.next();
}

#[test]
fn test_next_on_iter_with_no_elements() {
    struct TestKey;
    struct TestValue;

    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 0 },
        items: 0,
    };

    let mut iter = Iter {
        inner,
        marker: PhantomData,
    };

    let result = iter.next();
}


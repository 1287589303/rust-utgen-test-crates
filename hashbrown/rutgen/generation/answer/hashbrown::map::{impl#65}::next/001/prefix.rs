// Answer 0

#[test]
fn test_next_with_valid_entry() {
    struct TestKeys<'a> {
        inner: Iter<'a, i32, i32>,
    }

    struct TestValues<'a> {
        inner: Values<'a, i32, i32>,
    }

    let key_value_pair: (i32, i32) = (1, 100);
    let raw_iter = RawIter::new(vec![key_value_pair]);
    let iter = Iter { inner: raw_iter, marker: PhantomData };
    let mut values = TestValues { inner: Values { inner: iter } };

    values.inner.next();
}

#[test]
fn test_next_with_multiple_entries() {
    struct TestKeys<'a> {
        inner: Iter<'a, i32, i32>,
    }

    struct TestValues<'a> {
        inner: Values<'a, i32, i32>,
    }

    let key_value_pairs = vec![(1, 100), (2, 200), (3, 300)];
    let raw_iter = RawIter::new(key_value_pairs);
    let iter = Iter { inner: raw_iter, marker: PhantomData };
    let mut values = TestValues { inner: Values { inner: iter } };

    values.inner.next();
    values.inner.next();
    values.inner.next();
}


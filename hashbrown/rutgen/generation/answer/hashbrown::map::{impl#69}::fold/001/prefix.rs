// Answer 0

#[test]
fn test_fold_non_empty_values_mut() {
    struct TestKey;
    struct TestValue(i32);

    let mut items = vec![(TestKey, TestValue(1)), (TestKey, TestValue(2))];
    let mut values_mut = ValuesMut {
        inner: IterMut {
            inner: RawIter::new(&mut items),
            marker: PhantomData,
        },
    };

    let init = 0;
    let increment_fn = |acc: i32, v: &mut TestValue| {
        acc + v.0
    };

    values_mut.fold(init, increment_fn);
}

#[test]
fn test_fold_single_item_values_mut() {
    struct TestKey;
    struct TestValue(i32);

    let mut items = vec![(TestKey, TestValue(1))];
    let mut values_mut = ValuesMut {
        inner: IterMut {
            inner: RawIter::new(&mut items),
            marker: PhantomData,
        },
    };

    let init = 0;
    let single_fn = |acc: i32, v: &mut TestValue| {
        acc + v.0
    };

    values_mut.fold(init, single_fn);
}

#[test]
#[should_panic]
fn test_fold_with_panic_fn() {
    struct TestKey;
    struct TestValue(i32);

    let mut items = vec![(TestKey, TestValue(1)), (TestKey, TestValue(2))];
    let mut values_mut = ValuesMut {
        inner: IterMut {
            inner: RawIter::new(&mut items),
            marker: PhantomData,
        },
    };

    let init = 0;
    let panic_fn = |_, _: &mut TestValue| {
        panic!("Intentional panic for testing");
    };

    values_mut.fold(init, panic_fn);
}


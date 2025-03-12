// Answer 0

#[test]
fn test_clone_empty_iter() {
    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 0 },
        items: 0,
    };
    let iter = Iter {
        inner,
        marker: PhantomData,
    };
    let _cloned = iter.clone();
}

#[test]
fn test_clone_single_element_iter() {
    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 1 },
        items: 1,
    };
    let iter = Iter {
        inner,
        marker: PhantomData,
    };
    let _cloned = iter.clone();
}

#[test]
fn test_clone_multi_element_iter() {
    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 3 },
        items: 3,
    };
    let iter = Iter {
        inner,
        marker: PhantomData,
    };
    let _cloned = iter.clone();
}

#[test]
fn test_clone_iter_with_different_lifetime() {
    let inner = RawIter {
        iter: RawIterRange { start: 0, end: 2 },
        items: 2,
    };
    let iter: Iter<'static, i32, String> = Iter {
        inner,
        marker: PhantomData,
    };
    let _cloned = iter.clone();
}


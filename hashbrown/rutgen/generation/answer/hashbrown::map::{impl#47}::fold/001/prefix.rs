// Answer 0

#[test]
fn test_fold_empty_iter() {
    let iter: Iter<i32, i32> = Iter { inner: RawIter { iter: RawIterRange::Empty, items: 0 }, marker: PhantomData };
    let init_value = 0;
    let result = iter.fold(init_value, |acc, _| acc + 1);
}

#[test]
fn test_fold_single_element() {
    let iter: Iter<i32, i32> = Iter { inner: RawIter { iter: RawIterRange::from(vec![(1, 1)]), items: 1 }, marker: PhantomData };
    let init_value = 0;
    let result = iter.fold(init_value, |acc, (_, _)| acc + 1);
}

#[test]
fn test_fold_multiple_elements() {
    let iter_vec = vec![(1, 2), (3, 4), (5, 6)];
    let iter: Iter<i32, i32> = Iter { inner: RawIter { iter: RawIterRange::from(iter_vec.clone()), items: iter_vec.len() }, marker: PhantomData };
    let init_value = 0;
    let result = iter.fold(init_value, |acc, (_, _)| acc + 1);
}

#[test]
fn test_fold_with_custom_initial() {
    let iter_vec = vec![(10, 20), (30, 40)];
    let iter: Iter<i32, i32> = Iter { inner: RawIter { iter: RawIterRange::from(iter_vec.clone()), items: iter_vec.len() }, marker: PhantomData };
    let init_value = 5;
    let result = iter.fold(init_value, |acc, (_, _)| acc * 2);
}

#[test]
fn test_fold_edge_case() {
    let iter_vec = vec![(i32::MIN, i32::MAX)];
    let iter: Iter<i32, i32> = Iter { inner: RawIter { iter: RawIterRange::from(iter_vec.clone()), items: iter_vec.len() }, marker: PhantomData };
    let init_value = 1;
    let result = iter.fold(init_value, |acc, (_, v)| acc + v);
}


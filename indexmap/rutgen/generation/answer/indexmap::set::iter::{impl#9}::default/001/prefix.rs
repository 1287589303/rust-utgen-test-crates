// Answer 0

#[test]
fn test_default_empty_iter() {
    let iter: Iter<u32> = Iter::default();
    let _: SliceIter<'_, Bucket<u32>> = iter.iter;
}

#[test]
fn test_default_empty_iter_with_string() {
    let iter: Iter<String> = Iter::default();
    let _: SliceIter<'_, Bucket<String>> = iter.iter;
}

#[test]
fn test_default_empty_iter_with_tuple() {
    let iter: Iter<(i32, i32)> = Iter::default();
    let _: SliceIter<'_, Bucket<(i32, i32)>> = iter.iter;
}


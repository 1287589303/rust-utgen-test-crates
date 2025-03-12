// Answer 0

#[test]
fn test_into_iter_default() {
    let default_iter: IntoIter<i32, i32> = IntoIter::default();
    let _ = default_iter.iter.as_slice();
}

#[test]
fn test_into_iter_default_empty_iterator() {
    let default_iter: IntoIter<String, String> = IntoIter::default();
    let _ = default_iter.iter.as_slice();
}


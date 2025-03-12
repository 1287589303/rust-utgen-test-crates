// Answer 0

#[test]
fn test_default_iter_mut() {
    let iter_mut: IterMut<i32, i32> = Default::default();
}

#[test]
fn test_default_iter_mut_empty() {
    let iter_mut: IterMut<String, String> = Default::default();
}


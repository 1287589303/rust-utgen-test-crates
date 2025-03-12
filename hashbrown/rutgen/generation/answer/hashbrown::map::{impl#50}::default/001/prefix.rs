// Answer 0

#[test]
fn test_iter_mut_default() {
    let iter: IterMut<i32, i32> = Default::default();
    let _ = iter; // Ensure the variable is used to avoid unused variable warning
}

#[test]
fn test_iter_mut_default_empty() {
    let iter: IterMut<&str, &str> = Default::default();
    let _ = iter; // Ensure the variable is used to avoid unused variable warning
}


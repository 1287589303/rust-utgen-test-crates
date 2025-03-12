// Answer 0

#[test]
fn test_default_iter_creation() {
    let iter: Iter<()> = Default::default();
}

#[test]
fn test_default_iter_struct() {
    let iter: Iter<u32> = Default::default();
}

#[test]
fn test_default_iter_empty() {
    let iter: Iter<String> = Default::default();
}


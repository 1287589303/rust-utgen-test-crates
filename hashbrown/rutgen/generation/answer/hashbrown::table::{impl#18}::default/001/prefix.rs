// Answer 0

#[test]
fn test_iter_default() {
    let iter: Iter<u8> = Default::default();
}

#[test]
fn test_iter_default_empty() {
    let iter: Iter<String> = Default::default();
}

#[test]
fn test_iter_default_with_phantom() {
    let iter: Iter<&str> = Default::default();
}


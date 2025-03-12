// Answer 0

#[test]
fn test_keys_default() {
    let keys: Keys<usize, usize> = Keys::default();
    let _ = keys; // use to avoid unused variable warning
}

#[test]
fn test_iter_default() {
    let iter: Iter<usize> = Iter::default();
    let _ = iter; // use to avoid unused variable warning
}

#[test]
fn test_iter_kv_default() {
    let iter_kv: Iter<usize, usize> = Iter::default();
    let _ = iter_kv; // use to avoid unused variable warning
}


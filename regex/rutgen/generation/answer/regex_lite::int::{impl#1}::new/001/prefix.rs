// Answer 0

#[test]
fn test_new_zero() {
    let result = NonMaxUsize::new(0);
}

#[test]
fn test_new_one() {
    let result = NonMaxUsize::new(1);
}

#[test]
fn test_new_max_minus_one() {
    let result = NonMaxUsize::new(usize::MAX - 1);
}

#[test]
fn test_new_max() {
    let result = NonMaxUsize::new(usize::MAX);
}


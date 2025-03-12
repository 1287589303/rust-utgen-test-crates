// Answer 0

#[test]
fn test_get_with_min_value() {
    let non_max = NonMaxUsize::new(1).unwrap();
    let result = non_max.get();
}

#[test]
fn test_get_with_small_value() {
    let non_max = NonMaxUsize::new(2).unwrap();
    let result = non_max.get();
}

#[test]
fn test_get_with_middle_value() {
    let non_max = NonMaxUsize::new(100).unwrap();
    let result = non_max.get();
}

#[test]
fn test_get_with_large_value() {
    let non_max = NonMaxUsize::new(usize::MAX - 1).unwrap();
    let result = non_max.get();
}

#[test]
#[should_panic]
fn test_get_with_max_value() {
    let non_max = NonMaxUsize::new(usize::MAX).unwrap();
    let result = non_max.get();
}


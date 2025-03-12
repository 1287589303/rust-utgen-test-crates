// Answer 0

#[test]
fn test_set_range_excluded_bounds_1() {
    let mut input = Input::new(b"abcdef");
    input.set_range(1..=5);
}

#[test]
fn test_set_range_excluded_bounds_2() {
    let mut input = Input::new(b"abcdefg");
    input.set_range(3..=7);
}

#[test]
#[should_panic]
fn test_set_range_excluded_bounds_invalid() {
    let mut input = Input::new(b"abc");
    input.set_range(0..=usize::MAX);
}


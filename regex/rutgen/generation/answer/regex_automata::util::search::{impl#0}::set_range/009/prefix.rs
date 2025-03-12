// Answer 0

#[test]
fn test_set_range_inclusive_start_and_end() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_range(0..=5);
}

#[test]
fn test_set_range_inclusive_start_with_non_zero_end() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_range(1..=4);
}

#[test]
fn test_set_range_inclusive_start_with_zero_end_exclusive() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_range(0..=0);
}

#[test]
fn test_set_range_inclusive_start_with_edge_case() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_range(3..=5);
}

#[test]
fn test_set_range_full_range() {
    let mut input = Input::new(&b"foobar"[..]);
    input.set_range(0..=5);
}


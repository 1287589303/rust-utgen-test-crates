// Answer 0

#[test]
fn test_range_full_haystack() {
    let input = Input::new(b"foobar");
    let result = input.range(0..6);
}

#[test]
fn test_range_subslice() {
    let input = Input::new(b"foobar").range(2..4);
}

#[test]
fn test_range_boundary_start() {
    let input = Input::new(b"foobar").range(0..1);
}

#[test]
fn test_range_boundary_end() {
    let input = Input::new(b"foobar").range(5..6);
}

#[test]
fn test_range_empty() {
    let input = Input::new(b"foobar").range(2..2);
}

#[test]
#[should_panic]
fn test_range_out_of_bounds_start() {
    let input = Input::new(b"foobar").range(7..8);
}

#[test]
#[should_panic]
fn test_range_out_of_bounds_end() {
    let input = Input::new(b"foobar").range(6..7);
}

#[test]
#[should_panic]
fn test_range_invalid() {
    let input = Input::new(b"foobar").range(0..=usize::MAX);
}

#[test]
fn test_range_with_anchored() {
    let input = Input::new(b"foobar").anchored(Anchored::Yes).range(1..5);
}


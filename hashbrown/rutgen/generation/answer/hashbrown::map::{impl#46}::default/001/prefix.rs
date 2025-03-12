// Answer 0

#[test]
fn test_default_iter() {
    let iter: Iter<i32, i32> = Iter::default();
}

#[test]
fn test_default_iter_with_string() {
    let iter: Iter<String, String> = Iter::default();
}

#[test]
fn test_default_iter_with_floats() {
    let iter: Iter<f32, f32> = Iter::default();
}


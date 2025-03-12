// Answer 0

#[test]
fn test_display_empty_slice() {
    let empty = Empty;
    let mut output = String::new();
    let _ = empty.fmt(&mut output);
}

#[test]
fn test_display_non_empty_slice() {
    let non_empty = Empty;
    let mut output = String::new();
    let _ = non_empty.fmt(&mut output);
}


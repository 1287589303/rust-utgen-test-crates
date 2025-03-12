// Answer 0

#[test]
fn test_is_none_with_none_capture() {
    let which_captures = WhichCaptures::None;
    let result = which_captures.is_none();
}

#[test]
fn test_is_none_with_all_capture() {
    let which_captures = WhichCaptures::All;
    let result = which_captures.is_none();
}

#[test]
fn test_is_none_with_implicit_capture() {
    let which_captures = WhichCaptures::Implicit;
    let result = which_captures.is_none();
}


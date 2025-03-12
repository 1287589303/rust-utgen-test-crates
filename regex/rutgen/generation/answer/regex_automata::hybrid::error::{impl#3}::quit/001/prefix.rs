// Answer 0

#[test]
fn test_quit_with_zero() {
    let byte = 0;
    StartError::quit(byte);
}

#[test]
fn test_quit_with_middle() {
    let byte = 128;
    StartError::quit(byte);
}

#[test]
fn test_quit_with_max() {
    let byte = 255;
    StartError::quit(byte);
}


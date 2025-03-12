// Answer 0

#[test]
fn test_quit_with_zero() {
    let result = StartError::quit(0);
}

#[test]
fn test_quit_with_byte_min() {
    let result = StartError::quit(1);
}

#[test]
fn test_quit_with_byte_mid() {
    let result = StartError::quit(128);
}

#[test]
fn test_quit_with_byte_max() {
    let result = StartError::quit(255);
}


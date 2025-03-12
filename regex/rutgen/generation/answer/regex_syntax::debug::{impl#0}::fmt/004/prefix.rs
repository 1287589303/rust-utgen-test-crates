// Answer 0

#[test]
fn test_space_character() {
    let byte = super::Byte(b' ');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_null_character() {
    let byte = super::Byte(b'\x00');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_control_character() {
    let byte = super::Byte(b'\x1F');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_space_hex() {
    let byte = super::Byte(b'\x20');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_delete_character() {
    let byte = super::Byte(b'\x7F');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}


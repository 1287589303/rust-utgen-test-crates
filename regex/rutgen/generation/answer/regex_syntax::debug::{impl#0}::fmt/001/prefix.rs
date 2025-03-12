// Answer 0

#[test]
fn test_fmt_non_space_character_0() {
    let byte = super::Byte(0);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}

#[test]
fn test_fmt_non_space_character_1() {
    let byte = super::Byte(1);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}

#[test]
fn test_fmt_non_space_character_31() {
    let byte = super::Byte(31);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}

#[test]
fn test_fmt_non_space_character_33() {
    let byte = super::Byte(33);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}

#[test]
fn test_fmt_non_space_character_126() {
    let byte = super::Byte(126);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}

#[test]
fn test_fmt_non_space_character_255() {
    let byte = super::Byte(255);
    let mut buf = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut buf);
}


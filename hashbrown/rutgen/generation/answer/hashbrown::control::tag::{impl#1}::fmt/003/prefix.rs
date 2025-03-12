// Answer 0

#[test]
fn test_fmt_full_low() {
    let tag = Tag(0b0000_0000); // 0
    let mut formatter = fmt::Formatter::default();
    let _ = tag.fmt(&mut formatter);
}

#[test]
fn test_fmt_full_mid() {
    let tag = Tag(0b0111_1111); // 127
    let mut formatter = fmt::Formatter::default();
    let _ = tag.fmt(&mut formatter);
}

#[test]
fn test_fmt_full_high() {
    let tag = Tag(0b0111_1110); // 126
    let mut formatter = fmt::Formatter::default();
    let _ = tag.fmt(&mut formatter);
}


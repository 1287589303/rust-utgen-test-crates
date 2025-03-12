// Answer 0

#[test]
fn test_fmt_empty_byte_classes() {
    let mut byte_classes = ByteClasses::empty();
    let mut output = Vec::new();
    let formatter = &mut core::fmt::Formatter::new(&mut output);
    let _ = byte_classes.fmt(formatter);
}

#[test]
fn test_fmt_singleton_byte_classes() {
    let mut byte_classes = ByteClasses::singletons();
    let mut output = Vec::new();
    let formatter = &mut core::fmt::Formatter::new(&mut output);
    let _ = byte_classes.fmt(formatter);
}

#[test]
fn test_fmt_non_singleton_no_classes() {
    let mut byte_classes = ByteClasses::empty();
    let mut output = Vec::new();
    let formatter = &mut core::fmt::Formatter::new(&mut output);
    let _ = byte_classes.fmt(formatter);
}


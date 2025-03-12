// Answer 0

#[test]
fn test_maximum_len_unicode_empty() {
    let empty_class_unicode = Class::Unicode(ClassUnicode::empty());
    let _ = empty_class_unicode.maximum_len();
}

#[test]
fn test_maximum_len_unicode_single_byte() {
    let single_byte_class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('\u{0041}'..= '\u{0041}')]));
    let _ = single_byte_class_unicode.maximum_len();
}

#[test]
fn test_maximum_len_unicode_two_bytes() {
    let two_byte_class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('\u{00C1}'..= '\u{00C1}')]));
    let _ = two_byte_class_unicode.maximum_len();
}

#[test]
fn test_maximum_len_unicode_three_bytes() {
    let three_byte_class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('\u{0800}'..= '\u{0800}')]));
    let _ = three_byte_class_unicode.maximum_len();
}

#[test]
fn test_maximum_len_unicode_four_bytes() {
    let four_byte_class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('\u{10000}'..= '\u{10FFFF}')]));
    let _ = four_byte_class_unicode.maximum_len();
}

#[test]
fn test_maximum_len_unicode_unbounded_repeat() {
    let unbounded_class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new('\u{0041}'..= '\u{FFFF}')]));
    let _ = unbounded_class_unicode.maximum_len();
}


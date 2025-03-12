// Answer 0

#[test]
fn test_literal_single_interval_different_start_end() {
    let range = ClassUnicodeRange { start: 'a', end: 'b' };
    let class_unicode = ClassUnicode::new(vec![range]);

    let _result = class_unicode.literal();
}

#[test]
fn test_literal_single_interval_different_start_end_numeric() {
    let range = ClassUnicodeRange { start: '0', end: '1' };
    let class_unicode = ClassUnicode::new(vec![range]);

    let _result = class_unicode.literal();
}


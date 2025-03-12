// Answer 0

#[test]
fn test_literal_empty_class_unicode() {
    let class_unicode = ClassUnicode::empty();
    let _result = class_unicode.literal();
}

#[test]
fn test_literal_multiple_codepoints_class_unicode() {
    let class_unicode = {
        let mut range_set = IntervalSet { ranges: vec![], folded: false };
        range_set.ranges.push(ClassUnicodeRange { start: 'a', end: 'a' });
        range_set.ranges.push(ClassUnicodeRange { start: 'b', end: 'b' });
        ClassUnicode::new(range_set.ranges.into_iter())
    };
    let _result = class_unicode.literal();
}

#[test]
fn test_literal_non_singleton_class_unicode() {
    let class_unicode = {
        let mut range_set = IntervalSet { ranges: vec![], folded: false };
        range_set.ranges.push(ClassUnicodeRange { start: 'a', end: 'b' });
        ClassUnicode::new(range_set.ranges.into_iter())
    };
    let _result = class_unicode.literal();
}


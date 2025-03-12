// Answer 0

#[test]
fn test_case_fold_simple_with_ascii() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ];
    let mut class_unicode = ClassUnicode::new(ranges);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_non_ascii() {
    let ranges = vec![
        ClassUnicodeRange { start: 'α', end: 'ω' },
        ClassUnicodeRange { start: 'Α', end: 'Ω' },
    ];
    let mut class_unicode = ClassUnicode::new(ranges);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_mixed_ascii_non_ascii() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: 'α', end: 'ω' },
    ];
    let mut class_unicode = ClassUnicode::new(ranges);
    class_unicode.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_empty() {
    let ranges: Vec<ClassUnicodeRange> = vec![];
    let mut class_unicode = ClassUnicode::new(ranges);
    class_unicode.case_fold_simple();
}


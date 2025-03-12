// Answer 0

#[test]
fn test_class_unicode_iter_next_empty() {
    let intervals: Vec<ClassUnicodeRange> = vec![];
    let iter = IntervalSetIter(intervals.iter());
    let mut class_unicode_iter = ClassUnicodeIter(iter);
    let result = class_unicode_iter.next();
}

#[test]
fn test_class_unicode_iter_next_single_element() {
    let intervals: Vec<ClassUnicodeRange> = vec![ClassUnicodeRange { start: 'a', end: 'a' }];
    let iter = IntervalSetIter(intervals.iter());
    let mut class_unicode_iter = ClassUnicodeIter(iter);
    let result = class_unicode_iter.next();
}

#[test]
fn test_class_unicode_iter_next_multiple_elements() {
    let intervals: Vec<ClassUnicodeRange> = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'c', end: 'd' }
    ];
    let iter = IntervalSetIter(intervals.iter());
    let mut class_unicode_iter = ClassUnicodeIter(iter);
    let result1 = class_unicode_iter.next();
    let result2 = class_unicode_iter.next();
}

#[test]
fn test_class_unicode_iter_next_after_exhaustion() {
    let intervals: Vec<ClassUnicodeRange> = vec![
        ClassUnicodeRange { start: 'e', end: 'f' }
    ];
    let iter = IntervalSetIter(intervals.iter());
    let mut class_unicode_iter = ClassUnicodeIter(iter);
    let result1 = class_unicode_iter.next();
    let result2 = class_unicode_iter.next();
}


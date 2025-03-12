// Answer 0

#[test]
fn test_unwrap_class_unicode_literal() {
    let literal_frame = HirFrame::Literal(vec![b'a', b'b', b'c']);
    literal_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_class_bytes() {
    let class_bytes_frame = HirFrame::ClassBytes(hir::ClassBytes { set: IntervalSet::new() });
    class_bytes_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_repetition() {
    let repetition_frame = HirFrame::Repetition;
    repetition_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_group() {
    let group_frame = HirFrame::Group { old_flags: Flags::default() };
    group_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_concat() {
    let concat_frame = HirFrame::Concat;
    concat_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_alternation() {
    let alternation_frame = HirFrame::Alternation;
    alternation_frame.unwrap_class_unicode();
}

#[test]
fn test_unwrap_class_unicode_alternation_branch() {
    let alternation_branch_frame = HirFrame::AlternationBranch;
    alternation_branch_frame.unwrap_class_unicode();
}


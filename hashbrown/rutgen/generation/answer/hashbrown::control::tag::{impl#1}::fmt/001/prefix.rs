// Answer 0

#[test]
fn test_tag_fmt_special_empty() {
    let tag = Tag::EMPTY;
    let mut fmt_buffer = core::fmt::Formatter::new();
    let _ = tag.fmt(&mut fmt_buffer);
}

#[test]
fn test_tag_fmt_special_deleted() {
    let tag = Tag::DELETED;
    let mut fmt_buffer = core::fmt::Formatter::new();
    let _ = tag.fmt(&mut fmt_buffer);
}

#[test]
fn test_tag_fmt_special_range() {
    let tag = Tag(255);
    let mut fmt_buffer = core::fmt::Formatter::new();
    let _ = tag.fmt(&mut fmt_buffer);
}


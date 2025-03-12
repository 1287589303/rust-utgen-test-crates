// Answer 0

#[test]
fn test_is_special_zero() {
    let tag = Tag(0x00);
    tag.is_special();
}

#[test]
fn test_is_special_special() {
    let tag = Tag(0x80);
    tag.is_special();
}

#[test]
fn test_is_special_not_special() {
    let tag = Tag(0x7F);
    tag.is_special();
}

#[test]
fn test_is_special_max() {
    let tag = Tag(0xFF);
    tag.is_special();
}

#[test]
fn test_is_special_border() {
    let tag = Tag(0x81);
    tag.is_special();
}

#[test]
fn test_is_special_not_special_border() {
    let tag = Tag(0x7E);
    tag.is_special();
}


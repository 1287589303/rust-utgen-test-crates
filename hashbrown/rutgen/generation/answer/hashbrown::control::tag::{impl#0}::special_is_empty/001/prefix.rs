// Answer 0

#[test]
fn test_special_is_empty_true() {
    let tag = Tag(0b1000_0001); // special and empty
    let result = tag.special_is_empty();
}

#[test]
fn test_special_is_empty_false() {
    let tag = Tag(0b1000_0010); // special and not empty
    let result = tag.special_is_empty();
}

#[test]
fn test_special_is_empty_boundary_ones() {
    let tag = Tag(0b1111_1111); // special and not empty
    let result = tag.special_is_empty();
}


// Answer 0

#[test]
fn test_is_full_with_clear_top_bit_0() {
    let tag = Tag(0b0000_0000);
    tag.is_full();
}

#[test]
fn test_is_full_with_clear_top_bit_63() {
    let tag = Tag(0b0011_1111);
    tag.is_full();
}

#[test]
fn test_is_full_with_clear_top_bit_64() {
    let tag = Tag(0b0100_0000);
    tag.is_full();
}

#[test]
fn test_is_full_with_clear_top_bit_127() {
    let tag = Tag(0b0111_1111);
    tag.is_full();
}

#[test]
fn test_is_full_with_set_top_bit_128() {
    let tag = Tag(0b1000_0000);
    tag.is_full();
}

#[test]
fn test_is_full_with_set_top_bit_191() {
    let tag = Tag(0b1011_1111);
    tag.is_full();
}

#[test]
fn test_is_full_with_set_top_bit_255() {
    let tag = Tag(0b1111_1111);
    tag.is_full();
}


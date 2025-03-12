// Answer 0

#[test]
fn test_is_byte_with_equal_u8() {
    let unit = Unit::u8(100);
    assert!(unit.is_byte(100));
}

#[test]
fn test_is_byte_with_non_equal_u8() {
    let unit = Unit::u8(150);
    assert!(!unit.is_byte(100));
}

#[test]
fn test_is_byte_with_eoi() {
    let unit = Unit::eoi(256);
    assert!(!unit.is_byte(100));
}

#[test]
fn test_is_byte_with_boundary_u8_min() {
    let unit = Unit::u8(0);
    assert!(unit.is_byte(0));
}

#[test]
fn test_is_byte_with_boundary_u8_max() {
    let unit = Unit::u8(255);
    assert!(unit.is_byte(255));
}

#[test]
fn test_is_byte_with_boundary_u8_non_equal_min() {
    let unit = Unit::u8(0);
    assert!(!unit.is_byte(1));
}

#[test]
fn test_is_byte_with_boundary_u8_non_equal_max() {
    let unit = Unit::u8(255);
    assert!(!unit.is_byte(254));
}


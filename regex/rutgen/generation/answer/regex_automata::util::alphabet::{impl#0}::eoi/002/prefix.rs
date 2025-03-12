// Answer 0

#[test]
#[should_panic]
fn test_eoi_too_high() {
    let _unit = Unit::eoi(257);
}

#[test]
fn test_eoi_max_boundary() {
    let _unit = Unit::eoi(256);
}

#[test]
fn test_eoi_zero() {
    let _unit = Unit::eoi(0);
}

#[test]
fn test_eoi_within_range() {
    let _unit = Unit::eoi(128);
}


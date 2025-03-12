// Answer 0

#[test]
fn test_eoi_min() {
    let _unit = Unit::eoi(0);
}

#[test]
fn test_eoi_mid() {
    let _unit = Unit::eoi(128);
}

#[test]
fn test_eoi_max() {
    let _unit = Unit::eoi(256);
}

#[should_panic]
fn test_eoi_overflow() {
    let _unit = Unit::eoi(257);
}


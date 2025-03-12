// Answer 0

#[test]
fn test_as_eoi_with_u8_zero() {
    let unit = Unit(UnitKind::U8(0));
    unit.as_eoi();
}

#[test]
fn test_as_eoi_with_u8_one() {
    let unit = Unit(UnitKind::U8(1));
    unit.as_eoi();
}

#[test]
fn test_as_eoi_with_u8_max() {
    let unit = Unit(UnitKind::U8(255));
    unit.as_eoi();
}


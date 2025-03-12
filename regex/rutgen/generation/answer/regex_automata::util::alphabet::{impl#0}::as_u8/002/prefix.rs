// Answer 0

#[test]
fn test_as_u8_with_minimum_byte() {
    let unit = Unit(UnitKind::U8(0));
    unit.as_u8();
}

#[test]
fn test_as_u8_with_middle_byte() {
    let unit = Unit(UnitKind::U8(128));
    unit.as_u8();
}

#[test]
fn test_as_u8_with_maximum_byte() {
    let unit = Unit(UnitKind::U8(255));
    unit.as_u8();
}


// Answer 0

#[test]
fn test_as_usize_u8_zero() {
    let unit = Unit(UnitKind::U8(0));
    unit.as_usize();
}

#[test]
fn test_as_usize_u8_max() {
    let unit = Unit(UnitKind::U8(255));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_zero() {
    let unit = Unit(UnitKind::EOI(0));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_max() {
    let unit = Unit(UnitKind::EOI(256));
    unit.as_usize();
}


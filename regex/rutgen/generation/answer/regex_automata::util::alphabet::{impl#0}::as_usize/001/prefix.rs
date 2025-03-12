// Answer 0

#[test]
fn test_as_usize_eoi_zero() {
    let unit = Unit(UnitKind::EOI(0));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_one() {
    let unit = Unit(UnitKind::EOI(1));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_max() {
    let unit = Unit(UnitKind::EOI(256));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_mid() {
    let unit = Unit(UnitKind::EOI(128));
    unit.as_usize();
}

#[test]
fn test_as_usize_eoi_boundary() {
    let unit = Unit(UnitKind::EOI(255));
    unit.as_usize();
}


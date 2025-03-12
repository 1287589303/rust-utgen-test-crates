// Answer 0

#[test]
fn test_get_by_unit_eoi_zero() {
    let mut byte_classes = ByteClasses::empty();
    let unit = Unit(UnitKind::EOI(0));
    byte_classes.get_by_unit(unit);
}

#[test]
fn test_get_by_unit_eoi_one() {
    let mut byte_classes = ByteClasses::empty();
    let unit = Unit(UnitKind::EOI(1));
    byte_classes.get_by_unit(unit);
}

#[test]
fn test_get_by_unit_eoi_max() {
    let mut byte_classes = ByteClasses::empty();
    let unit = Unit(UnitKind::EOI(255));
    byte_classes.get_by_unit(unit);
}

#[test]
fn test_get_by_unit_eoi_mid() {
    let mut byte_classes = ByteClasses::empty();
    let unit = Unit(UnitKind::EOI(128));
    byte_classes.get_by_unit(unit);
}


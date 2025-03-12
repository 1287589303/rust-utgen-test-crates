// Answer 0

#[test]
fn test_unit_debug_eoi_boundary_case_0() {
    let unit = Unit(UnitKind::EOI(0));
    let _ = format!("{:?}", unit);
}

#[test]
fn test_unit_debug_eoi_boundary_case_256() {
    let unit = Unit(UnitKind::EOI(256));
    let _ = format!("{:?}", unit);
}

#[test]
fn test_unit_debug_eoi_inner_case_128() {
    let unit = Unit(UnitKind::EOI(128));
    let _ = format!("{:?}", unit);
}

#[test]
fn test_unit_debug_eoi_inner_case_64() {
    let unit = Unit(UnitKind::EOI(64));
    let _ = format!("{:?}", unit);
}

#[test]
fn test_unit_debug_eoi_inner_case_1() {
    let unit = Unit(UnitKind::EOI(1));
    let _ = format!("{:?}", unit);
}


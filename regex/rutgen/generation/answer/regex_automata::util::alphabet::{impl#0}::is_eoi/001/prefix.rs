// Answer 0

#[test]
fn test_is_eoi_with_unit_kind_u8() {
    let unit_0 = Unit(UnitKind::U8(0));
    let unit_255 = Unit(UnitKind::U8(255));

    unit_0.is_eoi();
    unit_255.is_eoi();
}

#[test]
fn test_is_eoi_with_unit_kind_eoi() {
    let unit_eoi_256 = Unit(UnitKind::EOI(256));
    
    unit_eoi_256.is_eoi();
}

#[test]
fn test_is_eoi_with_edge_case() {
    let unit_eoi_255 = Unit(UnitKind::EOI(255));
    
    unit_eoi_255.is_eoi();
}


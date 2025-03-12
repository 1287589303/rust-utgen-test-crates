// Answer 0

#[test]
fn test_fmt_with_u8_min() {
    let unit = Unit(UnitKind::U8(0));
    let mut formatter = core::fmt::Formatter::new();
    let _ = unit.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_u8_mid() {
    let unit = Unit(UnitKind::U8(128));
    let mut formatter = core::fmt::Formatter::new();
    let _ = unit.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_u8_max() {
    let unit = Unit(UnitKind::U8(255));
    let mut formatter = core::fmt::Formatter::new();
    let _ = unit.fmt(&mut formatter);
}


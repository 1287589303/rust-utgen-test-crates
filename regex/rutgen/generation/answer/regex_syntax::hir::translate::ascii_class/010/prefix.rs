// Answer 0

#[test]
fn test_ascii_class_cntrl() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Cntrl;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

#[test]
fn test_ascii_class_cntrl_boundary_low() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Cntrl;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

#[test]
fn test_ascii_class_cntrl_boundary_high() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Cntrl;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}


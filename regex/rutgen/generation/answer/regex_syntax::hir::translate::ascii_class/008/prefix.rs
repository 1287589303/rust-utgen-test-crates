// Answer 0

#[test]
fn test_asci_class_graph() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Graph;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}

#[test]
fn test_asci_class_graph_boundary() {
    use crate::ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Graph;
    let result: Vec<(u8, u8)> = ascii_class(&kind).collect();
}


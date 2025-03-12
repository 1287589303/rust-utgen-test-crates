// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    let mut output = Vec::new();
    let mut writer = Writer { wtr: &mut output };
    let ast = ast::ClassSetBinaryOpKind::Intersection;
    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    let mut output = Vec::new();
    let mut writer = Writer { wtr: &mut output };
    let ast = ast::ClassSetBinaryOpKind::Difference;
    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    let mut output = Vec::new();
    let mut writer = Writer { wtr: &mut output };
    let ast = ast::ClassSetBinaryOpKind::SymmetricDifference;
    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}


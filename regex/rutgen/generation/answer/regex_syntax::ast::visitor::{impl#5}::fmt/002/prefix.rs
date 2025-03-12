// Answer 0

#[test]
fn test_class_induct_binary_op_difference() {
    struct DummyAst;
    struct DummyClassSet;

    let lhs = Box::new(DummyClassSet);
    let rhs = Box::new(DummyClassSet);
    
    let binary_op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Difference,
        lhs,
        rhs,
    };

    let class_induct = ClassInduct::BinaryOp(&binary_op);

    let mut formatter = core::fmt::Formatter::new();
    let _ = class_induct.fmt(&mut formatter);
}


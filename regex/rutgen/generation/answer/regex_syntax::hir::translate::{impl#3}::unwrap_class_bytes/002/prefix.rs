// Answer 0

#[test]
fn test_unwrap_class_bytes_valid_case() {
    let class_bytes = hir::ClassBytes {
        set: IntervalSet::new(),
    };
    let frame = HirFrame::ClassBytes(class_bytes.clone());
    let result = frame.unwrap_class_bytes();
}

#[test]
#[should_panic]
fn test_unwrap_class_bytes_invalid_case() {
    let frame = HirFrame::Expr(Hir::default());
    let _result = frame.unwrap_class_bytes();
}


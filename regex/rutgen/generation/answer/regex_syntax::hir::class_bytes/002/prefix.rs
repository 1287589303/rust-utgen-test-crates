// Answer 0

#[test]
fn test_class_bytes_with_non_class_kind() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let hirs = vec![
        TestHir::new(HirKind::Empty),
        TestHir::new(HirKind::Look(Look)),
        TestHir::new(HirKind::Repetition(Repetition)),
    ];

    let result = class_bytes(&hirs);
}

#[test]
fn test_class_bytes_with_only_non_class_kinds() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let hirs = vec![
        TestHir::new(HirKind::Capture(Capture)),
        TestHir::new(HirKind::Concat(Vec::new())),
    ];

    let result = class_bytes(&hirs);
}


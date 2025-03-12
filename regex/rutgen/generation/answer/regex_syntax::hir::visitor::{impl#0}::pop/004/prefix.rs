// Answer 0

#[test]
fn test_pop_concat_non_empty_tail() {
    struct TestHir {
        kind: hir::HirKind,
        props: hir::Properties,
    }

    let hir1 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let hir2 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let tail: Vec<&Hir> = vec![&hir1, &hir2];

    let induct = Frame::Concat { head: &hir1, tail: &tail };

    let visitor = HeapVisitor::new();
    
    let result = visitor.pop(induct);
}

#[test]
fn test_pop_concat_tail_length_two() {
    struct TestHir {
        kind: hir::HirKind,
        props: hir::Properties,
    }

    let hir1 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let hir2 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let tail: Vec<&Hir> = vec![&hir2];

    let induct = Frame::Concat { head: &hir1, tail: &tail };

    let visitor = HeapVisitor::new();
    
    let result = visitor.pop(induct);
}

#[test]
fn test_pop_concat_tail_length_three() {
    struct TestHir {
        kind: hir::HirKind,
        props: hir::Properties,
    }

    let hir1 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let hir2 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let hir3 = TestHir { 
        kind: hir::HirKind::SomeKind,
        props: hir::Properties::default(),
    };

    let tail: Vec<&Hir> = vec![&hir2, &hir3];

    let induct = Frame::Concat { head: &hir1, tail: &tail };

    let visitor = HeapVisitor::new();
    
    let result = visitor.pop(induct);
}


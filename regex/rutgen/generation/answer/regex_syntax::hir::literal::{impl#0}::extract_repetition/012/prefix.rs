// Answer 0

#[test]
fn test_extract_repetition_case1() {
    struct MockRepetition {
        min: u32,
        max: Option<u32>,
        greedy: bool,
        sub: Hir,
    }

    struct MockHir {
        kind: hir::HirKind,
    }

    let sub_hir = MockHir {
        kind: hir::HirKind::Literal(hir::Literal(vec![b'a'])),
    };

    let repetition = MockRepetition {
        min: 3,
        max: Some(3),
        greedy: true,
        sub: sub_hir,
    };

    let extractor = Extractor::new()
        .limit_repeat(2);

    extractor.extract(&Hir { kind: hir::HirKind::Repetition(repetition) });
}

#[test]
fn test_extract_repetition_case2() {
    struct MockRepetition {
        min: u32,
        max: Option<u32>,
        greedy: bool,
        sub: Hir,
    }

    struct MockHir {
        kind: hir::HirKind,
    }

    let sub_hir = MockHir {
        kind: hir::HirKind::Literal(hir::Literal(vec![b'b'])),
    };

    let repetition = MockRepetition {
        min: 4,
        max: Some(4),
        greedy: false,
        sub: sub_hir,
    };

    let extractor = Extractor::new()
        .limit_repeat(3);

    extractor.extract(&Hir { kind: hir::HirKind::Repetition(repetition) });
}

#[test]
fn test_extract_repetition_case3() {
    struct MockRepetition {
        min: u32,
        max: Option<u32>,
        greedy: bool,
        sub: Hir,
    }

    struct MockHir {
        kind: hir::HirKind,
    }

    let sub_hir = MockHir {
        kind: hir::HirKind::Literal(hir::Literal(vec![b'c'])),
    };

    let repetition = MockRepetition {
        min: 5,
        max: Some(5),
        greedy: true,
        sub: sub_hir,
    };

    let extractor = Extractor::new()
        .limit_repeat(4);

    extractor.extract(&Hir { kind: hir::HirKind::Repetition(repetition) });
}


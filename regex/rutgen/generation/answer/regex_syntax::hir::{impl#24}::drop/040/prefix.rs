// Answer 0

#[test]
fn test_drop_repetition_empty_sub() {
    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties(Box::new(PropertiesI {})),
    };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 0,
            max: Some(0),
            greedy: false,
            sub: Box::new(empty_hir),
        }),
        props: Properties(Box::new(PropertiesI {})),
    };

    // Call the drop function implicitly by going out of scope
    let _ = repetition_hir;
}

#[test]
fn test_drop_repetition_empty_sub_greedy() {
    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties(Box::new(PropertiesI {})),
    };
    
    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 0,
            max: Some(0),
            greedy: true,
            sub: Box::new(empty_hir),
        }),
        props: Properties(Box::new(PropertiesI {})),
    };

    let _ = repetition_hir;
}


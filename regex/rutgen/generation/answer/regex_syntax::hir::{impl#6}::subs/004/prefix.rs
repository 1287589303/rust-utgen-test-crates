// Answer 0

#[test]
fn test_hirkind_subs_repetition_non_empty() {
    let sub_expression = Hir {
        kind: HirKind::Literal(Literal {
            bytes: vec![b'a'],
            exact: true,
        }),
        props: Properties::default(),
    };
    
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(sub_expression),
    };
    
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };
    
    let _result = hir.subs();
}

#[test]
fn test_hirkind_subs_repetition_non_empty_non_greedy() {
    let sub_expression = Hir {
        kind: HirKind::Capture(Capture {
            index: 1,
            name: Some(Box::from("group")),
            sub: Box::new(Hir {
                kind: HirKind::Literal(Literal {
                    bytes: vec![b'b'],
                    exact: true,
                }),
                props: Properties::default(),
            }),
        }),
        props: Properties::default(),
    };
    
    let repetition = Repetition {
        min: 0,
        max: Some(5),
        greedy: false,
        sub: Box::new(sub_expression),
    };
    
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };
    
    let _result = hir.subs();
}

#[test]
fn test_hirkind_subs_repetition_sub_concat() {
    let sub_expression1 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: vec![b'c'],
            exact: true,
        }),
        props: Properties::default(),
    };
    
    let sub_expression2 = Hir {
        kind: HirKind::Literal(Literal {
            bytes: vec![b'd'],
            exact: true,
        }),
        props: Properties::default(),
    };
    
    let repetition = Repetition {
        min: 2,
        max: None,
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Concat(vec![sub_expression1, sub_expression2]),
            props: Properties::default(),
        }),
    };
    
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties::default(),
    };
    
    let _result = hir.subs();
}


// Answer 0

#[test]
fn test_from_alternation_literals_valid_case() {
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: vec![Prop {
            look_set: vec![],
            explicit_captures_len: 0,
            is_alternation_literal: true,
        }],
        config: Config {
            match_kind: MatchKind::LeftmostFirst,
        },
    }));
    
    let literals = (0..3000).map(|i| {
        Hir::from_literal(format!("literal_{}", i).as_bytes())
    }).collect::<Vec<Hir>>();
    
    let hirs: &[&Hir] = &literals[..1];

    let result = Pre::from_alternation_literals(&info, hirs);
}

#[test]
fn test_from_alternation_literals_edge_case() {
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: vec![Prop {
            look_set: vec![],
            explicit_captures_len: 0,
            is_alternation_literal: true,
        }],
        config: Config {
            match_kind: MatchKind::LeftmostFirst,
        },
    }));

    let literals = (0..3000).map(|i| {
        Hir::from_literal(format!("literal_{}", i).as_bytes())
    }).collect::<Vec<Hir>>();
    
    let hirs: &[&Hir] = &literals[..1];

    let result = Pre::from_alternation_literals(&info, hirs);
}


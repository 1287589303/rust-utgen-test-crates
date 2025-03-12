// Answer 0

#[test]
fn test_lift_common_prefix_single_element_non_concat() {
    let hirs = vec![Hir {
        kind: HirKind::Literal(Literal::new("test".to_string())),
        props: Properties::default(),
    }];
    let result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_element_class() {
    let hirs = vec![Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        props: Properties::default(),
    }];
    let result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_element_look() {
    let hirs = vec![Hir {
        kind: HirKind::Look(Look::new()),
        props: Properties::default(),
    }];
    let result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_element_repetition() {
    let hirs = vec![Hir {
        kind: HirKind::Repetition(Repetition::new()),
        props: Properties::default(),
    }];
    let result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_single_element_capture() {
    let hirs = vec![Hir {
        kind: HirKind::Capture(Capture::new()),
        props: Properties::default(),
    }];
    let result = lift_common_prefix(hirs);
}


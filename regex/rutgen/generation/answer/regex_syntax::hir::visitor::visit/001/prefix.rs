// Answer 0

#[test]
fn test_visit_with_empty_hir() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_node(&mut self, _node: &Hir) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties::new(),
    };

    let _ = visit(&empty_hir, DummyVisitor);
}

#[test]
fn test_visit_with_single_node() {
    struct SingleNodeVisitor;

    impl Visitor for SingleNodeVisitor {
        type Output = ();
        type Err = ();

        fn visit_node(&mut self, _node: &Hir) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let single_node_hir = Hir {
        kind: HirKind::Node,
        props: Properties::new(),
    };

    let _ = visit(&single_node_hir, SingleNodeVisitor);
}

#[test]
fn test_visit_with_multiple_nodes() {
    struct MultipleNodesVisitor;

    impl Visitor for MultipleNodesVisitor {
        type Output = ();
        type Err = ();

        fn visit_node(&mut self, _node: &Hir) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let multiple_nodes_hir = Hir {
        kind: HirKind::Sequence(vec![
            Hir { kind: HirKind::Node, props: Properties::new() },
            Hir { kind: HirKind::Node, props: Properties::new() },
        ]),
        props: Properties::new(),
    };

    let _ = visit(&multiple_nodes_hir, MultipleNodesVisitor);
}

#[test]
fn test_visit_with_error_returning_visitor() {
    struct ErrorReturningVisitor;

    impl Visitor for ErrorReturningVisitor {
        type Output = ();
        type Err = ();

        fn visit_node(&mut self, _node: &Hir) -> Result<Self::Output, Self::Err> {
            Err(())
        }
    }

    let error_hir = Hir {
        kind: HirKind::Node,
        props: Properties::new(),
    };

    let _ = visit(&error_hir, ErrorReturningVisitor);
}

#[test]
fn test_visit_with_large_hir() {
    struct LargeHirVisitor;

    impl Visitor for LargeHirVisitor {
        type Output = ();
        type Err = ();

        fn visit_node(&mut self, _node: &Hir) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let large_hir = Hir {
        kind: HirKind::Sequence((0..1000).map(|_| Hir { kind: HirKind::Node, props: Properties::new() }).collect()),
        props: Properties::new(),
    };

    let _ = visit(&large_hir, LargeHirVisitor);
}


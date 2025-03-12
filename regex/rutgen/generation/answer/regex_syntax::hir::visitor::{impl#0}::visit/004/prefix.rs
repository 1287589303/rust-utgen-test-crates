// Answer 0

#[test]
fn test_visit_with_repetition() {
    struct TestVisitor {
        output: Vec<()>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.output.push(());
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.output.push(());
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }
    let repetition = hir::Repetition { sub: Hir { kind: HirKind::Capture(...), props: Properties::new() } };
    let hir = Hir { kind: HirKind::Repetition(repetition), props: Properties::new() };
    
    let mut visitor = TestVisitor { output: Vec::new() };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&hir, visitor).unwrap();
}

#[test]
fn test_visit_with_concat() {
    struct TestVisitor {
        output: Vec<()>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let child_hir_1 = Hir { kind: HirKind::Capture(...), props: Properties::new() };
    let child_hir_2 = Hir { kind: HirKind::Capture(...), props: Properties::new() };
    let hir = Hir { kind: HirKind::Concat(vec![child_hir_1, child_hir_2]), props: Properties::new() };

    let mut visitor = TestVisitor { output: Vec::new() };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&hir, visitor).unwrap();
}

#[test]
fn test_visit_with_alternation() {
    struct TestVisitor {
        output: Vec<()>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let child_hir_1 = Hir { kind: HirKind::Repetition(...), props: Properties::new() };
    let child_hir_2 = Hir { kind: HirKind::Repetition(...), props: Properties::new() };
    let hir = Hir { kind: HirKind::Alternation(vec![child_hir_1, child_hir_2]), props: Properties::new() };

    let mut visitor = TestVisitor { output: Vec::new() };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&hir, visitor).unwrap();
}


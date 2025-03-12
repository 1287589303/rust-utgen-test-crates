// Answer 0

#[test]
fn test_visit_with_error_in_visit_pre() {
    struct TestVisitor {
        should_return_err: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.should_return_err {
                Err(())
            } else {
                Ok(())
            }
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
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor { should_return_err: true };
    let repetitions = Hir { kind: HirKind::Repetition(repetition_instance), props: Properties::default() };
    let captures = Hir { kind: HirKind::Capture(capture_instance), props: Properties::default() };
    let empty_concat = Hir { kind: HirKind::Concat(vec![]), props: Properties::default() };
    let empty_alternation = Hir { kind: HirKind::Alternation(vec![]), props: Properties::default() };

    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&repetitions, visitor).unwrap_err();
    heap_visitor.visit(&captures, visitor).unwrap_err();
    heap_visitor.visit(&empty_concat, visitor).unwrap_err();
    heap_visitor.visit(&empty_alternation, visitor).unwrap_err();
}

#[test]
fn test_visit_with_empty_alternation() {
    struct TestVisitor {
        return_for_visit_pre: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.return_for_visit_pre {
                Ok(())
            } else {
                Err(())
            }
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
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let visitor = TestVisitor { return_for_visit_pre: true };
    let empty_alternation = Hir { kind: HirKind::Alternation(vec![]), props: Properties::default() };
    
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&empty_alternation, visitor).unwrap();
}

#[test]
fn test_visit_with_empty_concat() {
    struct TestVisitor {
        return_for_visit_pre: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.return_for_visit_pre {
                Ok(())
            } else {
                Err(())
            }
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
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let visitor = TestVisitor { return_for_visit_pre: true };
    let empty_concat = Hir { kind: HirKind::Concat(vec![]), props: Properties::default() };
    
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&empty_concat, visitor).unwrap();
}

#[test]
fn test_visit_with_various_hir_kinds() {
    struct TestVisitor {
        visit_pre_return: Result<(), ()>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.visit_pre_return.clone()
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
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let results = vec![
        Ok(()),  // For a valid case
        Err(()), // For an error case
    ];

    for result in results {
        let visitor = TestVisitor { visit_pre_return: result.clone() };
        let repetition_hir = Hir { kind: HirKind::Repetition(repetition_instance), props: Properties::default() };
        let capture_hir = Hir { kind: HirKind::Capture(capture_instance), props: Properties::default() };

        let mut heap_visitor = HeapVisitor::new();
        
        if result.is_ok() {
            heap_visitor.visit(&repetition_hir, visitor).expect("Should not return an error");
            heap_visitor.visit(&capture_hir, visitor).expect("Should not return an error");
        } else {
            heap_visitor.visit(&repetition_hir, visitor).unwrap_err();
            heap_visitor.visit(&capture_hir, visitor).unwrap_err();
        }
    }
}


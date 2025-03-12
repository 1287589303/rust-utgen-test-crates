// Answer 0

#[test]
fn test_visit_class_set_binary_op_pre_unicode() {
    struct TestVisitor {
        translator: Translator,
    }
    
    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = Error;
        
        fn finish(self) -> Result<Hir> {
            Ok(Hir { kind: HirKind::Any, props: Properties::default() })
        }
        
        fn visit_class_set_binary_op_pre(&mut self, op: &ast::ClassSetBinaryOp) -> Result<()> {
            // Function under test
            if self.flags().unicode() {
                let cls = hir::ClassUnicode::empty();
                self.push(HirFrame::ClassUnicode(cls));
            } else {
                let cls = hir::ClassBytes::empty();
                self.push(HirFrame::ClassBytes(cls));
            }
            Ok(())
        }
        
        fn flags(&self) -> Flags {
            Flags { unicode: Some(true), ..Flags::default() }
        }

        fn push(&self, frame: HirFrame) {
            self.translator.stack.borrow_mut().push(frame);
        }
    }

    let translator = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }), 
        utf8: true, 
        line_terminator: b'\n' 
    };

    let mut visitor = TestVisitor { translator };
    let op = ast::ClassSetBinaryOp { span: Span::default(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::new_empty()), rhs: Box::new(ClassSet::new_empty()) };
    
    let _ = visitor.visit_class_set_binary_op_pre(&op);
}

#[test]
fn test_visit_class_set_binary_op_pre_non_unicode() {
    struct TestVisitor {
        translator: Translator,
    }
    
    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = Error;
        
        fn finish(self) -> Result<Hir> {
            Ok(Hir { kind: HirKind::Any, props: Properties::default() })
        }
        
        fn visit_class_set_binary_op_pre(&mut self, op: &ast::ClassSetBinaryOp) -> Result<()> {
            // Function under test
            if self.flags().unicode() {
                let cls = hir::ClassUnicode::empty();
                self.push(HirFrame::ClassUnicode(cls));
            } else {
                let cls = hir::ClassBytes::empty();
                self.push(HirFrame::ClassBytes(cls));
            }
            Ok(())
        }
        
        fn flags(&self) -> Flags {
            Flags { unicode: Some(false), ..Flags::default() }
        }

        fn push(&self, frame: HirFrame) {
            self.translator.stack.borrow_mut().push(frame);
        }
    }

    let translator = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }), 
        utf8: true, 
        line_terminator: b'\n' 
    };

    let mut visitor = TestVisitor { translator };
    let op = ast::ClassSetBinaryOp { span: Span::default(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::new_empty()), rhs: Box::new(ClassSet::new_empty()) };
    
    let _ = visitor.visit_class_set_binary_op_pre(&op);
}


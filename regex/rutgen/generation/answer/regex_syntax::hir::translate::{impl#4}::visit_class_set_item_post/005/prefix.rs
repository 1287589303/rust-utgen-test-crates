// Answer 0

#[test]
fn test_visit_class_set_item_post_with_non_empty_class_bytes() {
    struct DummyVisitor {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
    }

    impl DummyVisitor {
        fn new() -> Self {
            Self {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
            }
        }
        
        fn pop(&self) -> Option<HirFrame> {
            self.stack.borrow_mut().pop()
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
        
        fn bytes_fold_and_negate(
            &self,
            span: &Span,
            negated: bool,
            class: &mut ClassBytes,
        ) -> Result<()> {
            if negated {
                class.negate();
            }
            Ok(())
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    let mut visitor = DummyVisitor::new();
    
    let span = Span { start: Position::new(0), end: Position::new(10) };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(0, 255)]);
    visitor.push(HirFrame::ClassBytes(class_bytes.clone()));

    let class_set_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span,
        negated: false,
        kind: ast::ClassSet::Union,
    }));

    visitor.visit_class_set_item_post(&class_set_item).unwrap();
} 

#[test]
fn test_visit_class_set_item_post_with_negated_class_bytes() {
    struct DummyVisitor {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
    }

    impl DummyVisitor {
        fn new() -> Self {
            Self {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
            }
        }
        
        fn pop(&self) -> Option<HirFrame> {
            self.stack.borrow_mut().pop()
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
        
        fn bytes_fold_and_negate(
            &self,
            span: &Span,
            negated: bool,
            class: &mut ClassBytes,
        ) -> Result<()> {
            if negated {
                class.negate();
            }
            Ok(())
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    let mut visitor = DummyVisitor::new();
    
    let span = Span { start: Position::new(0), end: Position::new(10) };
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(0, 100)]);
    visitor.push(HirFrame::ClassBytes(class_bytes.clone()));

    let class_set_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span,
        negated: true,
        kind: ast::ClassSet::Union,
    }));

    visitor.visit_class_set_item_post(&class_set_item).unwrap();
}


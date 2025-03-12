// Answer 0

#[test]
fn test_lazy_fmt_with_initialized_value() {
    struct TestDebug(i32);
    
    impl fmt::Debug for TestDebug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestDebug({})", self.0)
        }
    }
    
    let once_cell = OnceCell(Imp::new());
    once_cell.set(Some(TestDebug(42))).unwrap();
    let lazy = Lazy { cell: once_cell, init: Cell::new(None) };
    let mut formatter = fmt::Formatter::new();
    
    lazy.fmt(&mut formatter);
}

#[test]
fn test_lazy_fmt_with_none_value() {
    struct TestDebug(i32);
    
    impl fmt::Debug for TestDebug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestDebug({})", self.0)
        }
    }
    
    let once_cell = OnceCell(Imp::new());
    let lazy = Lazy { cell: once_cell, init: Cell::new(None) };
    let mut formatter = fmt::Formatter::new();
    
    lazy.fmt(&mut formatter);
}

#[test]
fn test_lazy_fmt_with_empty_value() {
    struct TestEmpty;
    
    impl fmt::Debug for TestEmpty {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestEmpty")
        }
    }
    
    let once_cell = OnceCell(Imp::new());
    let lazy = Lazy { cell: once_cell, init: Cell::new(None) };
    let mut formatter = fmt::Formatter::new();
    
    lazy.fmt(&mut formatter);
}

#[test]
fn test_lazy_fmt_with_different_debug_types() {
    struct AnotherDebug(String);
    
    impl fmt::Debug for AnotherDebug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherDebug({})", self.0)
        }
    }
    
    let once_cell = OnceCell(Imp::new());
    once_cell.set(Some(AnotherDebug("Hello".to_string()))).unwrap();
    let lazy = Lazy { cell: once_cell, init: Cell::new(None) };
    let mut formatter = fmt::Formatter::new();
    
    lazy.fmt(&mut formatter);
}


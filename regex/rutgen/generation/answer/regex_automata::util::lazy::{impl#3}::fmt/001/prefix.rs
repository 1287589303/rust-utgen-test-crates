// Answer 0

#[test]
fn test_fmt_with_debug_instance() {
    struct Debuggable;
    impl fmt::Debug for Debuggable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("DebuggableInstance")
        }
    }

    let debuggable_instance = Debuggable;
    let lazy_instance = Lazy(lazy::Lazy {
        state: AtomicU8::new(0),
        create: Cell::new(Some(|| debuggable_instance)),
        data: Cell::new(MaybeUninit::uninit()),
    });
    let mut formatter = fmt::Formatter::new();
    let _ = lazy_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_empty_debug_instance() {
    struct EmptyDebuggable;
    impl fmt::Debug for EmptyDebuggable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("")
        }
    }

    let empty_instance = EmptyDebuggable;
    let lazy_instance = Lazy(lazy::Lazy {
        state: AtomicU8::new(0),
        create: Cell::new(Some(|| empty_instance)),
        data: Cell::new(MaybeUninit::uninit()),
    });
    let mut formatter = fmt::Formatter::new();
    let _ = lazy_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_complex_debug_instance() {
    struct ComplexDebuggable;
    impl fmt::Debug for ComplexDebuggable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Complex: {{ field1: {}, field2: {} }}", 42, "hello")
        }
    }

    let complex_instance = ComplexDebuggable;
    let lazy_instance = Lazy(lazy::Lazy {
        state: AtomicU8::new(0),
        create: Cell::new(Some(|| complex_instance)),
        data: Cell::new(MaybeUninit::uninit()),
    });
    let mut formatter = fmt::Formatter::new();
    let _ = lazy_instance.fmt(&mut formatter);
}


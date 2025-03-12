// Answer 0

#[test]
fn test_array64_debug_impl() {
    struct DebugType;

    impl fmt::Debug for DebugType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DebugType")
        }
    }

    let array: Array64<DebugType> = Array64([DebugType; 64]);
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);

    array.fmt(formatter);
}

#[test]
fn test_array64_debug_impl_with_another_type() {
    struct AnotherDebugType;

    impl fmt::Debug for AnotherDebugType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherDebugType")
        }
    }

    let array: Array64<AnotherDebugType> = Array64([AnotherDebugType; 64]);
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);

    array.fmt(formatter);
}


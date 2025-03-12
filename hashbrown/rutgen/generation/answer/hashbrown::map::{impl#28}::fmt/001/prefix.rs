// Answer 0

#[test]
fn test_fmt_with_debug_key() {
    struct DebugKey {
        value: i32,
    }

    impl Debug for DebugKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DebugKey({})", self.value)
        }
    }

    let keys: Vec<(DebugKey, &str)> = vec![
        (DebugKey { value: 1 }, "value1"),
        (DebugKey { value: 2 }, "value2"),
    ];

    let raw_iter = RawIter::from_keys_values(keys);
    let iter = Iter { inner: raw_iter, marker: PhantomData };
    let keys_struct = Keys { inner: iter };

    let _ = fmt::Debug::fmt(&keys_struct, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_empty_keys() {
    struct DebugKey {
        value: i32,
    }

    impl Debug for DebugKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DebugKey({})", self.value)
        }
    }

    let keys: Vec<(DebugKey, &str)> = vec![];
    let raw_iter = RawIter::from_keys_values(keys);
    let iter = Iter { inner: raw_iter, marker: PhantomData };
    let keys_struct = Keys { inner: iter };

    let _ = fmt::Debug::fmt(&keys_struct, &mut fmt::Formatter::new());
}


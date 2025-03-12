// Answer 0

#[test]
fn test_poll_with_initialized_value() {
    struct TestStruct {
        value: i32,
    }

    let lazy = Lazy {
        state: AtomicU8::new(2), // LAZY_STATE_DONE
        create: Cell::new(Some(|| TestStruct { value: 42 })),
        data: Cell::new(MaybeUninit::new(TestStruct { value: 42 })),
    };

    let result = lazy.poll();
}

#[test]
fn test_poll_with_different_initialized_value() {
    struct AnotherStruct {
        value: String,
    }

    let lazy = Lazy {
        state: AtomicU8::new(2), // LAZY_STATE_DONE
        create: Cell::new(Some(|| AnotherStruct { value: String::from("Hello") })),
        data: Cell::new(MaybeUninit::new(AnotherStruct { value: String::from("Hello") })),
    };

    let result = lazy.poll();
}

#[test]
fn test_poll_with_edge_case_initialized_value() {
    struct EdgeStruct {
        value: f64,
    }

    let lazy = Lazy {
        state: AtomicU8::new(2), // LAZY_STATE_DONE
        create: Cell::new(Some(|| EdgeStruct { value: 0.0 })),
        data: Cell::new(MaybeUninit::new(EdgeStruct { value: 0.0 })),
    };

    let result = lazy.poll();
}


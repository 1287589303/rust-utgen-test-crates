// Answer 0

#[test]
fn test_drop_with_non_null_pointer() {
    struct DummyData {
        value: i32,
    }

    let data = Box::new(DummyData { value: 42 });
    let lazy: Lazy<DummyData, fn() -> DummyData> = Lazy {
        state: AtomicU8::new(LAZY_STATE_DONE),
        create: Cell::new(None),
        data: Cell::new(MaybeUninit::new(Box::into_raw(data))),
    };

    unsafe {
        lazy.drop();
    }
}

#[test]
fn test_drop_with_another_non_null_pointer() {
    struct AnotherDummyData {
        value: String,
    }

    let data = Box::new(AnotherDummyData { value: String::from("Hello") });
    let lazy: Lazy<AnotherDummyData, fn() -> AnotherDummyData> = Lazy {
        state: AtomicU8::new(LAZY_STATE_DONE),
        create: Cell::new(None),
        data: Cell::new(MaybeUninit::new(Box::into_raw(data))),
    };

    unsafe {
        lazy.drop();
    }
}

#[test]
fn test_drop_with_different_data_type() {
    struct DifferentData {
        value: f64,
    }

    let data = Box::new(DifferentData { value: 3.14 });
    let lazy: Lazy<DifferentData, fn() -> DifferentData> = Lazy {
        state: AtomicU8::new(LAZY_STATE_DONE),
        create: Cell::new(None),
        data: Cell::new(MaybeUninit::new(Box::into_raw(data))),
    };

    unsafe {
        lazy.drop();
    }
}


// Answer 0

#[test]
fn test_get_when_data_is_ready() {
    struct TestStruct {
        value: i32,
    }

    let lazy_value = Lazy {
        state: AtomicU8::new(LAZY_STATE_DONE),
        create: Cell::new(Some(|| TestStruct { value: 42 })),
        data: Cell::new(MaybeUninit::uninit()),
    };

    let ptr = Box::into_raw(Box::new(TestStruct { value: 42 }));
    lazy_value.data.set(MaybeUninit::new(ptr));

    let result = lazy_value.get();
}

#[test]
fn test_get_when_data_is_not_initialized_yet() {
    struct TestStruct {
        value: i32,
    }

    let lazy_value = Lazy {
        state: AtomicU8::new(LAZY_STATE_INIT),
        create: Cell::new(Some(|| TestStruct { value: 100 })),
        data: Cell::new(MaybeUninit::uninit()),
    };

    let result = lazy_value.get();
}

#[test]
fn test_get_when_data_is_being_initialized() {
    struct TestStruct {
        value: i32,
    }

    let lazy_value = Lazy {
        state: AtomicU8::new(LAZY_STATE_BUSY),
        create: Cell::new(Some(|| TestStruct { value: 200 })),
        data: Cell::new(MaybeUninit::uninit()),
    };

    let ptr = Box::into_raw(Box::new(TestStruct { value: 200 }));
    let _ = lazy_value.data.compare_exchange(
        core::ptr::null_mut(),
        ptr,
        Ordering::Acquire,
        Ordering::Acquire,
    );

    let result = lazy_value.get();
}


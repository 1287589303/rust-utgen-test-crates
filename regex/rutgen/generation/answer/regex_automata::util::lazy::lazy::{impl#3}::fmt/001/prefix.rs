// Answer 0

#[test]
fn test_fmt_lazy_state_init() {
    struct TestStruct;
    let lazy = Lazy::<TestStruct, _> {
        state: AtomicU8::new(LAZY_STATE_INIT),
        create: Cell::new(Some(|| TestStruct)),
        data: Cell::new(MaybeUninit::uninit()),
    };
    let mut formatter = fmt::Formatter::new();
    let _ = lazy.fmt(&mut formatter);
}

#[test]
fn test_fmt_lazy_state_busy() {
    struct TestStruct;
    let lazy = Lazy::<TestStruct, _> {
        state: AtomicU8::new(LAZY_STATE_BUSY),
        create: Cell::new(Some(|| TestStruct)),
        data: Cell::new(MaybeUninit::uninit()),
    };
    let mut formatter = fmt::Formatter::new();
    let _ = lazy.fmt(&mut formatter);
}

#[test]
fn test_fmt_lazy_state_done() {
    struct TestStruct;
    let lazy = Lazy::<TestStruct, _> {
        state: AtomicU8::new(LAZY_STATE_DONE),
        create: Cell::new(Some(|| TestStruct)),
        data: Cell::new(MaybeUninit::new(TestStruct)),
    };
    let mut formatter = fmt::Formatter::new();
    let _ = lazy.fmt(&mut formatter);
}


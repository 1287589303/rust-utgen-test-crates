// Answer 0

#[test]
fn test_once_cell_with_sync_struct() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    share(&once_cell::sync::OnceCell::<S>::new());
}

#[test]
fn test_lazy_with_sync_struct() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
}


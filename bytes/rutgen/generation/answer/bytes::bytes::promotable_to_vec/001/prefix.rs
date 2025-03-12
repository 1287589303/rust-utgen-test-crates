// Answer 0

#[test]
fn test_promotable_to_vec_kind_arc() {
    use core::ptr::null_mut;
    use std::sync::Arc;
    
    struct TestData {
        data: AtomicPtr<()>,
    }

    let shared_data = Arc::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });
    let data = Arc::into_raw(shared_data);
    let atomic_ptr = AtomicPtr::new(data as *mut ());

    let buffer: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ptr: *const u8 = buffer.as_ptr();
    let len: usize = 10;

    unsafe fn mock_f(shared: *mut ()) -> *mut u8 {
        let shared: *mut Shared = shared.cast();
        (*shared).buf
    }

    let _result = unsafe { promotable_to_vec(&atomic_ptr, ptr, len, mock_f) };
}

#[test]
fn test_promotable_to_vec_kind_arc_empty() {
    use core::ptr::null_mut;
    use std::sync::Arc;

    struct TestData {
        data: AtomicPtr<()>,
    }

    let shared_data = Arc::new(Shared {
        buf: Box::into_raw(Box::new([])) as *mut u8,
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    });
    let data = Arc::into_raw(shared_data);
    let atomic_ptr = AtomicPtr::new(data as *mut ());

    let buffer: [u8; 0] = [];
    let ptr: *const u8 = buffer.as_ptr();
    let len: usize = 0;

    unsafe fn mock_f(shared: *mut ()) -> *mut u8 {
        let shared: *mut Shared = shared.cast();
        (*shared).buf
    }

    let _result = unsafe { promotable_to_vec(&atomic_ptr, ptr, len, mock_f) };
}

#[test]
fn test_promotable_to_vec_kind_arc_partial_fill() {
    use core::ptr::null_mut;
    use std::sync::Arc;

    struct TestData {
        data: AtomicPtr<()>,
    }

    let shared_data = Arc::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });
    let data = Arc::into_raw(shared_data);
    let atomic_ptr = AtomicPtr::new(data as *mut ());

    let buffer: [u8; 5] = [1, 2, 3, 4, 5];
    let ptr: *const u8 = buffer.as_ptr();
    let len: usize = 5;

    unsafe fn mock_f(shared: *mut ()) -> *mut u8 {
        let shared: *mut Shared = shared.cast();
        (*shared).buf
    }

    let _result = unsafe { promotable_to_vec(&atomic_ptr, ptr, len, mock_f) };
}


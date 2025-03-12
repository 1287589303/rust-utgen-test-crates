// Answer 0

#[test]
fn test_promotable_to_vec_with_valid_inputs_and_kind_vec() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared_data = TestShared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    };

    let shared_ptr = NonNull::new(&mut shared_data as *mut _ as *mut ()).unwrap();
    let atomic_ptr: AtomicPtr<()> = AtomicPtr::new(shared_ptr.as_ptr());

    let data = &atomic_ptr;
    let buffer_ptr: *const u8 = shared_data.buf;
    let length: usize = 5;

    extern "C" fn buffer_handler(shared: *mut ()) -> *mut u8 {
        let shared_ref = unsafe { &mut *(shared as *mut TestShared) };
        shared_ref.buf
    }

    let _result = unsafe {
        promotable_to_vec(data, buffer_ptr, length, buffer_handler)
    };
}

#[test]
fn test_promotable_to_vec_with_positive_length() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared_data = TestShared {
        buf: Box::into_raw(Box::new([1u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    };

    let shared_ptr = NonNull::new(&mut shared_data as *mut _ as *mut ()).unwrap();
    let atomic_ptr: AtomicPtr<()> = AtomicPtr::new(shared_ptr.as_ptr());

    let data = &atomic_ptr;
    let buffer_ptr: *const u8 = shared_data.buf;
    let length: usize = 1;

    extern "C" fn buffer_handler(shared: *mut ()) -> *mut u8 {
        let shared_ref = unsafe { &mut *(shared as *mut TestShared) };
        shared_ref.buf
    }

    let _result = unsafe {
        promotable_to_vec(data, buffer_ptr, length, buffer_handler)
    };
}

#[test]
fn test_promotable_to_vec_with_large_length() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared_data = TestShared {
        buf: Box::into_raw(Box::new([2u8; 100])) as *mut u8,
        cap: 100,
        ref_cnt: AtomicUsize::new(1),
    };

    let shared_ptr = NonNull::new(&mut shared_data as *mut _ as *mut ()).unwrap();
    let atomic_ptr: AtomicPtr<()> = AtomicPtr::new(shared_ptr.as_ptr());

    let data = &atomic_ptr;
    let buffer_ptr: *const u8 = shared_data.buf;
    let length: usize = 10;

    extern "C" fn buffer_handler(shared: *mut ()) -> *mut u8 {
        let shared_ref = unsafe { &mut *(shared as *mut TestShared) };
        shared_ref.buf
    }

    let _result = unsafe {
        promotable_to_vec(data, buffer_ptr, length, buffer_handler)
    };
}


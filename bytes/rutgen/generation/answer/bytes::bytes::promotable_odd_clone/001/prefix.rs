// Answer 0

#[test]
fn test_promotable_odd_clone_arc() {
    let buf = Box::new([0u8; 10]);
    let arc_shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(buf) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(arc_shared as *mut _);
    let ptr: *const u8 = (arc_shared as *mut Shared).cast::<u8>().add(0);
    let len: usize = 10;

    unsafe {
        promotable_odd_clone(&data, ptr, len);
    }

    // Clean up
    unsafe {
        let _ = Box::from_raw(data.load(Ordering::Acquire));
    }
}

#[test]
fn test_promotable_odd_clone_arc_non_zero_length() {
    let buf = Box::new([1u8; 5]);
    let arc_shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(buf) as *mut u8,
        cap: 5,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(arc_shared as *mut _);
    let ptr: *const u8 = (arc_shared as *mut Shared).cast::<u8>().add(0);
    let len: usize = 5;

    unsafe {
        promotable_odd_clone(&data, ptr, len);
    }

    // Clean up
    unsafe {
        let _ = Box::from_raw(data.load(Ordering::Acquire));
    }
}

#[test]
fn test_promotable_odd_clone_arc_large_length() {
    let buf = Box::new([2u8; 100]);
    let arc_shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(buf) as *mut u8,
        cap: 100,
        ref_cnt: AtomicUsize::new(1),
    }));
    let data = AtomicPtr::new(arc_shared as *mut _);
    let ptr: *const u8 = (arc_shared as *mut Shared).cast::<u8>().add(0);
    let len: usize = 100;

    unsafe {
        promotable_odd_clone(&data, ptr, len);
    }

    // Clean up
    unsafe {
        let _ = Box::from_raw(data.load(Ordering::Acquire));
    }
}


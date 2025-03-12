// Answer 0

#[test]
fn test_drop_arc_valid_shared() {
    let mut shared = Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    };

    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: 5,
        cap: 10,
        data: &mut shared as *mut Shared,
    };

    // Call drop method
    let _ = std::mem::ManuallyDrop::new(bytes_mut);
} 

#[test]
fn test_drop_arc_valid_shared_with_high_ref_count() {
    let mut shared = Shared {
        buf: Box::into_raw(Box::new([0u8; 15])) as *mut u8,
        cap: 15,
        ref_cnt: AtomicUsize::new(5), // Reference count greater than 1
    };

    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: 7,
        cap: 15,
        data: &mut shared as *mut Shared,
    };

    // Call drop method
    let _ = std::mem::ManuallyDrop::new(bytes_mut);
}


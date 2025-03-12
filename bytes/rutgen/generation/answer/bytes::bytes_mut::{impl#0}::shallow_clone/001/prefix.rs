// Answer 0

#[test]
fn test_shallow_clone_with_arc_kind() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let data_ptr = NonNull::new(&mut bytes_mut as *mut _ as *mut u8).unwrap();
    let shared = Shared {
        buf: data_ptr.as_ptr(),
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    };
    
    unsafe {
        bytes_mut.data = &shared as *const _ as *mut Shared;
        let _clone = bytes_mut.shallow_clone();
    }
}

#[test]
fn test_shallow_clone_with_arc_kind_multiple_references() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    let data_ptr = NonNull::new(&mut bytes_mut as *mut _ as *mut u8).unwrap();
    let shared = Shared {
        buf: data_ptr.as_ptr(),
        cap: 20,
        ref_cnt: AtomicUsize::new(2),
    };

    unsafe {
        bytes_mut.data = &shared as *const _ as *mut Shared;
        let _clone = bytes_mut.shallow_clone();
    }
}


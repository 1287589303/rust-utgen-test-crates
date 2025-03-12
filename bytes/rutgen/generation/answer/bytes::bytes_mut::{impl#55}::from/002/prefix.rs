// Answer 0

#[test]
fn test_from_unique_shared() {
    let mut bytes = BytesMut::new();
    let shared = Box::into_raw(Box::new(Shared {
        vec: vec![1, 2, 3, 4],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));
    
    unsafe {
        bytes.data = shared as *mut Shared as *mut _;
        bytes.ptr = NonNull::new_unchecked(bytes.data);
        bytes.len = 4;
        bytes.cap = 4;

        let vec: Vec<u8> = Vec::from(bytes);
    }
}

#[test]
fn test_from_unique_shared_large_vector() {
    let mut bytes = BytesMut::new();
    let shared = Box::into_raw(Box::new(Shared {
        vec: vec![10; 1024],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));
    
    unsafe {
        bytes.data = shared as *mut Shared as *mut _;
        bytes.ptr = NonNull::new_unchecked(bytes.data);
        bytes.len = 1024;
        bytes.cap = 1024;

        let vec: Vec<u8> = Vec::from(bytes);
    }
}

#[test]
fn test_from_unique_shared_empty() {
    let mut bytes = BytesMut::new();
    let shared = Box::into_raw(Box::new(Shared {
        vec: vec![],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    unsafe {
        bytes.data = shared as *mut Shared as *mut _;
        bytes.ptr = NonNull::new_unchecked(bytes.data);
        bytes.len = 0;
        bytes.cap = 0;

        let vec: Vec<u8> = Vec::from(bytes);
    }
}

#[test]
fn test_from_unique_shared_non_empty() {
    let mut bytes = BytesMut::new();
    let shared = Box::into_raw(Box::new(Shared {
        vec: vec![5, 6, 7],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    unsafe {
        bytes.data = shared as *mut Shared as *mut _;
        bytes.ptr = NonNull::new_unchecked(bytes.data);
        bytes.len = 3;
        bytes.cap = 3;

        let vec: Vec<u8> = Vec::from(bytes);
    }
}


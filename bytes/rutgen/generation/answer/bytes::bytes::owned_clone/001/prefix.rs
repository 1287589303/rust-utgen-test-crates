// Answer 0

#[test]
fn test_owned_clone_with_high_ref_count() {
    use core::sync::atomic::AtomicPtr;
    
    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }
    
    unsafe {
        let owned_lifetime = OwnedLifetime {
            ref_cnt: AtomicUsize::new(usize::MAX >> 1 + 1),
            drop: std::ptr::drop_in_place,
        };

        let layout = std::alloc::Layout::new::<OwnedLifetime>();
        let ptr = std::alloc::alloc(layout);
        ptr::write(ptr as *mut OwnedLifetime, owned_lifetime);
        let atomic_ptr = AtomicPtr::new(ptr);

        let data: &AtomicPtr<()> = &atomic_ptr;
        let test_ptr: *const u8 = &0u8;
        let len: usize = 1;

        let _ = owned_clone(data, test_ptr, len);
    }
}


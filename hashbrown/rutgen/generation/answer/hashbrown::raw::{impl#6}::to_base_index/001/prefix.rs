// Answer 0

#[test]
fn test_to_base_index_zero_sized_type() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base_ptr: *mut ZST = &mut ZST as *mut ZST;
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket_ptr: *mut ZST = &mut ZST as *mut ZST;
    let bucket = unsafe { Bucket::from_base_index(NonNull::new_unchecked(bucket_ptr), 0) };

    let result = unsafe { bucket.to_base_index(base_non_null) };
}

#[test]
fn test_to_base_index_zero_sized_type_offset_one() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base_ptr: *mut ZST = &mut ZST as *mut ZST;
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket_ptr: *mut ZST = unsafe { base_ptr.add(1) };
    let bucket = unsafe { Bucket::from_base_index(NonNull::new_unchecked(bucket_ptr), 1) };

    let result = unsafe { bucket.to_base_index(base_non_null) };
}

#[test]
fn test_to_base_index_zero_sized_type_multiple_offsets() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base_ptr: *mut ZST = &mut ZST as *mut ZST;
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket_ptr: *mut ZST = unsafe { base_ptr.add(5) };
    let bucket = unsafe { Bucket::from_base_index(NonNull::new_unchecked(bucket_ptr), 5) };

    let result = unsafe { bucket.to_base_index(base_non_null) };
}


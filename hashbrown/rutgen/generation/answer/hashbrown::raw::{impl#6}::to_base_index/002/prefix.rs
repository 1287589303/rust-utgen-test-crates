// Answer 0

#[test]
unsafe fn test_to_base_index_non_zero_sized() {
    struct NonZeroSize;
    const IS_ZERO_SIZED: bool = false;
    
    let mut data = Box::new([NonZeroSize; 10]);
    let base = NonNull::new_unchecked(data.as_mut() as *mut _ as *mut NonZeroSize);
    let bucket = Bucket {
        ptr: NonNull::new_unchecked(base.as_ptr().sub(3)),
    };

    let index = bucket.to_base_index(base);
}

#[test]
unsafe fn test_to_base_index_first_element() {
    struct NonZeroSize;
    const IS_ZERO_SIZED: bool = false;

    let mut data = Box::new([NonZeroSize; 5]);
    let base = NonNull::new_unchecked(data.as_mut() as *mut _ as *mut NonZeroSize);
    let bucket = Bucket {
        ptr: NonNull::new_unchecked(base.as_ptr()),
    };

    let index = bucket.to_base_index(base);
}

#[test]
unsafe fn test_to_base_index_last_element() {
    struct NonZeroSize;
    const IS_ZERO_SIZED: bool = false;

    let mut data = Box::new([NonZeroSize; 10]);
    let base = NonNull::new_unchecked(data.as_mut() as *mut _ as *mut NonZeroSize);
    let bucket = Bucket {
        ptr: NonNull::new_unchecked(base.as_ptr().sub(9)),
    };

    let index = bucket.to_base_index(base);
}


// Answer 0

#[test]
unsafe fn test_is_bucket_full_valid_index_zero() {
    let alloc = Global;
    let table = RawTable::<u32, _>::new_in(alloc);
    let index = 0;
    let result = table.is_bucket_full(index);
}

#[test]
unsafe fn test_is_bucket_full_valid_index_middle() {
    let alloc = Global;
    let table = RawTable::<u32, _>::new_in(alloc);
    let index = table.buckets() / 2;
    let result = table.is_bucket_full(index);
}

#[test]
unsafe fn test_is_bucket_full_valid_index_max() {
    let alloc = Global;
    let table = RawTable::<u32, _>::new_in(alloc);
    let index = table.buckets() - 1;
    let result = table.is_bucket_full(index);
}

#[test]
#[should_panic]
unsafe fn test_is_bucket_full_invalid_index() {
    let alloc = Global;
    let table = RawTable::<u32, _>::new_in(alloc);
    let index = table.buckets(); // This index is out of bounds
    let result = table.is_bucket_full(index);
}


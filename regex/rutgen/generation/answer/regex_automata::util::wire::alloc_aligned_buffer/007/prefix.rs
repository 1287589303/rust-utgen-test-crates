// Answer 0

#[test]
fn test_alloc_aligned_buffer_zero_size() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(0);
    let align = core::mem::align_of::<StateID>();
    assert!(buf.len() == 0);
    assert!(padding == 0);
    assert!(buf.as_ptr().as_usize() % align == 0);
}

#[test]
fn test_alloc_aligned_buffer_max_size() {
    let size = 8;
    let (buf, padding) = alloc_aligned_buffer::<StateID>(size);
    let align = core::mem::align_of::<StateID>();
    assert!(buf.len() == size + 1); // padding must push size to 9
    assert!(padding > 7);
    assert!(buf.as_ptr().as_usize() % align == 0);
}

#[test]
fn test_alloc_aligned_buffer_small_size() {
    let size = 1;
    let (buf, padding) = alloc_aligned_buffer::<StateID>(size);
    let align = core::mem::align_of::<StateID>();
    assert!(buf.len() == size + 8); // padding must push size to 9
    assert!(padding > 7);
    assert!(buf.as_ptr().as_usize() % align == 0);
}


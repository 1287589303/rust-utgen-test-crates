// Answer 0

#[test]
fn test_alloc_aligned_buffer_size_0() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(0);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert_eq!(padding, 0);
}

#[test]
fn test_alloc_aligned_buffer_size_1() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(1);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert_eq!(padding, 0);
}

#[test]
fn test_alloc_aligned_buffer_size_2() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(2);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert_eq!(padding, 0);
}

#[test]
fn test_alloc_aligned_buffer_size_3() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(3);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert_eq!(padding, 0);
}

#[test]
fn test_alloc_aligned_buffer_size_4() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(4);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert_eq!(padding, 0);
}

#[test]
fn test_alloc_aligned_buffer_size_5() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(5);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert!(padding > 0);
}

#[test]
fn test_alloc_aligned_buffer_size_6() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(6);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert!(padding > 0);
}

#[test]
fn test_alloc_aligned_buffer_size_7() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(7);
    let align = core::mem::align_of::<StateID>();
    let address = buf.as_ptr().as_usize();
    assert!(address % align == 0);
    assert!(padding > 0);
}


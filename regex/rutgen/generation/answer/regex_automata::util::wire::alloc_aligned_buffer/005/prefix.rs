// Answer 0

#[test]
fn test_alloc_aligned_buffer_size_zero() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(0);
    assert_eq!(0, buf.len());
    assert_eq!(0, padding);
}

#[test]
fn test_alloc_aligned_buffer_size_one() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(1);
    assert_eq!(1, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_two() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(2);
    assert_eq!(2, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_seven() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(7);
    assert_eq!(7, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_eight() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(8);
    assert_eq!(8, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_nine() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(9);
    assert_eq!(9, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_fifteen() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(15);
    assert_eq!(15, buf[padding..].len());
}

#[test]
fn test_alloc_aligned_buffer_size_sixteen() {
    let (buf, padding) = alloc_aligned_buffer::<StateID>(16);
    assert_eq!(16, buf[padding..].len());
}


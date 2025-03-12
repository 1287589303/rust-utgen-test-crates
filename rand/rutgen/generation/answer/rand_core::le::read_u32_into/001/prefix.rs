// Answer 0

#[test]
fn test_read_u32_into_exact_length() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0];
    let mut dst: &mut [u32] = &mut [0; 2];
    read_u32_into(src, dst);
}

#[test]
fn test_read_u32_into_exceeding_length() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0, 3];
    let mut dst: &mut [u32] = &mut [0; 2];
    read_u32_into(src, dst);
}

#[test]
fn test_read_u32_into_zero_length_dst() {
    let src: &[u8] = &[];
    let mut dst: &mut [u32] = &mut [];
    read_u32_into(src, dst);
}

#[should_panic]
fn test_read_u32_into_insufficient_src_length() {
    let src: &[u8] = &[1, 0, 0, 0];
    let mut dst: &mut [u32] = &mut [0; 1];
    read_u32_into(src, dst);
}


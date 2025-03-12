// Answer 0

#[test]
fn test_read_u32_into_min_length() {
    let src: [u8; 4] = [1, 0, 0, 0]; // 4 * 1
    let mut dst: [u32; 1] = [0];
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_zero_length_dst() {
    let src: [u8; 4] = [1, 0, 0, 0]; // src.len() == 4, dst.len() == 0
    let mut dst: [u32; 0] = [];
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_boundary_min() {
    let src: [u8; 8] = [1, 0, 0, 0, 2, 0, 0, 0]; // 4 * 2
    let mut dst: [u32; 2] = [0, 0];
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_excessive_length() {
    let src: [u8; 12] = [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]; // 4 * 3
    let mut dst: [u32; 3] = [0, 0, 0];
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_insufficient_src_length() {
    let src: [u8; 4] = [1, 0, 0, 0]; // src.len() < 4 * dst.len()
    let mut dst: [u32; 2] = [0, 0]; // 4 * 2 = 8; src.len() is 4
    read_u32_into(&src, &mut dst);
}


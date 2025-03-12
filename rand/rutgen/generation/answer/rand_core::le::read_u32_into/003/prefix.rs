// Answer 0

#[test]
#[should_panic]
fn test_read_u32_into_src_length_less_than_dst() {
    let src: &[u8] = &[];
    let mut dst = vec![0u32; 1];
    read_u32_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_src_length_equals_three_dst_length_one() {
    let src: &[u8] = &[1, 2, 3];
    let mut dst = vec![0u32; 1];
    read_u32_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_src_length_equals_five_dst_length_two() {
    let src: &[u8] = &[1, 2, 3, 4, 5];
    let mut dst = vec![0u32; 2];
    read_u32_into(src, &mut dst);
}


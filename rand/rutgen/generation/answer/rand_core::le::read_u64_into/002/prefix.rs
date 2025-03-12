// Answer 0

#[test]
fn test_read_u64_into_zero_length() {
    let src: &[u8] = &[];
    let mut dst: &mut [u64] = &mut [];
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_one_u64() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: &mut [u64] = &mut [0];
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_two_u64s() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: &mut [u64] = &mut [0, 0];
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_three_u64s() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: &mut [u64] = &mut [0, 0, 0];
    read_u64_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_zero_src_non_empty_dst() {
    let src: &[u8] = &[];
    let mut dst: &mut [u64] = &mut [0];
    read_u64_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_src_less_than_dst() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0];
    let mut dst: &mut [u64] = &mut [0, 0];
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_dst_zero_length() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: &mut [u64] = &mut [];
    read_u64_into(src, &mut dst);
}


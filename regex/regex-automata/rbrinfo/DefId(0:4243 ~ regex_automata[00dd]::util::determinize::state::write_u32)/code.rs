fn write_u32(dst: &mut Vec<u8>, n: u32) {
    use crate::util::wire::NE;

    let start = dst.len();
    dst.extend(core::iter::repeat(0).take(mem::size_of::<u32>()));
    NE::write_u32(n, &mut dst[start..]);
}
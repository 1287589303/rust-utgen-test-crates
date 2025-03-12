fn read_vari32(data: &[u8]) -> (i32, usize) {
    let (un, i) = read_varu32(data);
    let mut n = i32::from_bits(un >> 1);
    if un & 1 != 0 {
        n = !n;
    }
    (n, i)
}
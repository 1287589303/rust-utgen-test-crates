fn read_varu32(data: &[u8]) -> (u32, usize) {
    // N.B. We can assume correctness here since we know that all varuints are
    // written with write_varu32. Hence, the 'as' uses and unchecked arithmetic
    // is all okay.
    let mut n: u32 = 0;
    let mut shift: u32 = 0;
    for (i, &b) in data.iter().enumerate() {
        if b < 0b1000_0000 {
            return (n | (u32::from(b) << shift), i + 1);
        }
        n |= (u32::from(b) & 0b0111_1111) << shift;
        shift += 7;
    }
    (0, 0)
}
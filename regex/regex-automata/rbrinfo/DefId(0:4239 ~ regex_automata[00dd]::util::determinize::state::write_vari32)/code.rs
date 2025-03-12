fn write_vari32(data: &mut Vec<u8>, n: i32) {
    let mut un = n.to_bits() << 1;
    if n < 0 {
        un = !un;
    }
    write_varu32(data, un)
}
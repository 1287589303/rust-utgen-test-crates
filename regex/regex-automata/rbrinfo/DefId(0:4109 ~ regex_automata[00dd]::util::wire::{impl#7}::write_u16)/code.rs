fn write_u16(n: u16, dst: &mut [u8]) {
        dst[..2].copy_from_slice(&n.to_be_bytes());
    }
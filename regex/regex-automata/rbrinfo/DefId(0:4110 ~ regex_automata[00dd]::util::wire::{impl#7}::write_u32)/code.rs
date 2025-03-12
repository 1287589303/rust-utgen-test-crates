fn write_u32(n: u32, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&n.to_be_bytes());
    }
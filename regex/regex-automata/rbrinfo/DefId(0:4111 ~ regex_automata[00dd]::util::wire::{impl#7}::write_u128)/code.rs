fn write_u128(n: u128, dst: &mut [u8]) {
        dst[..16].copy_from_slice(&n.to_be_bytes());
    }
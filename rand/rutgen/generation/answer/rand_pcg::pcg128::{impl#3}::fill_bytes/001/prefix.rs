// Answer 0

#[test]
fn test_fill_bytes_empty_slice() {
    let mut rng = Lcg128Xsl64 {
        state: 0x1234_5678_90AB_CDEF_U128,
        increment: 0xFEDC_BA98_7654_3210_U128,
    };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_slice() {
    let mut rng = Lcg128Xsl64 {
        state: 0xABCD_EF01_2345_6789_U128,
        increment: 0x9876_5432_10FE_DCBA_U128,
    };
    let mut dest: [u8; 1] = [0];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_mid_size_slice() {
    let mut rng = Lcg128Xsl64 {
        state: 0x1A2B_3C4D_5E6F_7A8B_U128,
        increment: 0xB9B8_B7B6_B5B4_B3B2_U128,
    };
    let mut dest: [u8; 512] = [0; 512];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_large_slice() {
    let mut rng = Lcg128Xsl64 {
        state: 0xDEAD_BEEF_F00D_CAFE_U128,
        increment: 0xCAFE_BABE_F00D_DEAD_U128,
    };
    let mut dest: [u8; 1024] = [0; 1024];
    rng.fill_bytes(&mut dest);
}


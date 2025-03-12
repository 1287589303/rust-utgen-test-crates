// Answer 0

#[test]
fn test_fill_bytes_min_length() {
    let mut rng = Mcg128Xsl64 { state: 1 };
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_standard_length_16() {
    let mut rng = Mcg128Xsl64 { state: 2 };
    let mut dest = [0u8; 16];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_standard_length_32() {
    let mut rng = Mcg128Xsl64 { state: 3 };
    let mut dest = [0u8; 32];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_large_length() {
    let mut rng = Mcg128Xsl64 { state: 4 };
    let mut dest = [0u8; 128];
    rng.fill_bytes(&mut dest);
}


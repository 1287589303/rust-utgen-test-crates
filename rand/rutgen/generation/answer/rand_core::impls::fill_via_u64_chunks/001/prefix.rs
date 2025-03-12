// Answer 0

#[test]
fn test_fill_via_u64_chunks_small() {
    let mut src: [u64; 1] = [0x0102030405060708];
    let mut dest: [u8; 8] = [0; 8];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_boundary() {
    let mut src: [u64; 2] = [0x0102030405060708, 0x090A0B0C0D0E0F10];
    let mut dest: [u8; 16] = [0; 16];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_full() {
    let mut src: [u64; 4] = [0x0102030405060708, 0x090A0B0C0D0E0F10, 0x1112131415161718, 0x191A1B1C1D1E1F20];
    let mut dest: [u8; 32] = [0; 32];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_partial() {
    let mut src: [u64; 3] = [0x0102030405060708, 0x090A0B0C0D0E0F10, 0x1112131415161718];
    let mut dest: [u8; 20] = [0; 20];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
}

#[test]
fn test_fill_via_u64_chunks_exceeding_dest() {
    let mut src: [u64; 5] = [0x0102030405060708, 0x090A0B0C0D0E0F10, 0x1112131415161718, 0x191A1B1C1D1E1F20, 0x2122232425262728];
    let mut dest: [u8; 48] = [0; 48];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
}


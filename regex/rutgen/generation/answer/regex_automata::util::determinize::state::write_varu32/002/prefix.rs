// Answer 0

#[test]
fn test_write_varu32_zero() {
    let mut data = Vec::new();
    let n: u32 = 0;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_one() {
    let mut data = Vec::new();
    let n: u32 = 1;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_127() {
    let mut data = Vec::new();
    let n: u32 = 127;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_128() {
    let mut data = Vec::new();
    let n: u32 = 128;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_255() {
    let mut data = Vec::new();
    let n: u32 = 255;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_256() {
    let mut data = Vec::new();
    let n: u32 = 256;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_16383() {
    let mut data = Vec::new();
    let n: u32 = 16383;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_16384() {
    let mut data = Vec::new();
    let n: u32 = 16384;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_2097151() {
    let mut data = Vec::new();
    let n: u32 = 2097151;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_max_2097152() {
    let mut data = Vec::new();
    let n: u32 = 2097152;
    write_varu32(&mut data, n);
}


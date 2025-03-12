// Answer 0

#[test]
fn test_write_u16_minimum_value() {
    let mut dst = [0u8; 2];
    let n: u16 = 0;
    BE::write_u16(n, &mut dst);
}

#[test]
fn test_write_u16_middle_value() {
    let mut dst = [0u8; 2];
    let n: u16 = 32768;
    BE::write_u16(n, &mut dst);
}

#[test]
fn test_write_u16_maximum_value() {
    let mut dst = [0u8; 2];
    let n: u16 = 65535;
    BE::write_u16(n, &mut dst);
}

#[test]
fn test_write_u16_boundary_value_1() {
    let mut dst = [0u8; 2];
    let n: u16 = 1;
    BE::write_u16(n, &mut dst);
}

#[test]
fn test_write_u16_boundary_value_2() {
    let mut dst = [0u8; 2];
    let n: u16 = 65534;
    BE::write_u16(n, &mut dst);
}


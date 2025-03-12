// Answer 0

#[test]
fn test_write_u16_zero() {
    let mut buffer = [0u8; 2];
    LE::write_u16(0, &mut buffer);
}

#[test]
fn test_write_u16_max() {
    let mut buffer = [0u8; 2];
    LE::write_u16(65535, &mut buffer);
}

#[test]
fn test_write_u16_mid() {
    let mut buffer = [0u8; 2];
    LE::write_u16(32768, &mut buffer);
}

#[test]
fn test_write_u16_edge_case_1() {
    let mut buffer = [0u8; 2];
    LE::write_u16(1, &mut buffer);
}

#[test]
fn test_write_u16_edge_case_2() {
    let mut buffer = [0u8; 2];
    LE::write_u16(65534, &mut buffer);
}


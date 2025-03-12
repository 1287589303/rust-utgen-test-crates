// Answer 0

#[test]
fn test_read_vari32_zero_length() {
    let data: &[u8] = &[];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_0() {
    let data: &[u8] = &[0];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_2() {
    let data: &[u8] = &[2];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_4() {
    let data: &[u8] = &[4];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_6() {
    let data: &[u8] = &[6];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_8() {
    let data: &[u8] = &[8];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_10() {
    let data: &[u8] = &[10];
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_even_un_f() {
    let data: &[u8] = &[0x0F];
    let result = read_vari32(data);
}


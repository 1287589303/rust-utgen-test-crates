// Answer 0

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    let n: i32 = 0;
    write_vari32(&mut data, n);
}

#[test]
fn test_write_vari32_positive_one() {
    let mut data = Vec::new();
    let n: i32 = 1;
    write_vari32(&mut data, n);
}

#[test]
fn test_write_vari32_positive_max() {
    let mut data = Vec::new();
    let n: i32 = 2_147_483_647;
    write_vari32(&mut data, n);
}


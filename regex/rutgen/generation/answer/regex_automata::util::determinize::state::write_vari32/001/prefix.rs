// Answer 0

#[test]
fn test_write_vari32_negative_one() {
    let mut data = Vec::new();
    write_vari32(&mut data, -1);
}

#[test]
fn test_write_vari32_negative_two() {
    let mut data = Vec::new();
    write_vari32(&mut data, -2);
}

#[test]
fn test_write_vari32_negative_max() {
    let mut data = Vec::new();
    write_vari32(&mut data, -2147483648);
}


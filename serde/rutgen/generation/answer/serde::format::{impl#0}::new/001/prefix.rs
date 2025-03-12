// Answer 0

#[test]
fn test_buf_new_empty() {
    let mut bytes: [u8; 0] = [];
    let buf = Buf::new(&mut bytes);
}

#[test]
fn test_buf_new_small() {
    let mut bytes: [u8; 1] = [1];
    let buf = Buf::new(&mut bytes);
}

#[test]
fn test_buf_new_moderate() {
    let mut bytes: [u8; 10] = [0; 10];
    let buf = Buf::new(&mut bytes);
}

#[test]
fn test_buf_new_large() {
    let mut bytes: [u8; 1024] = [0; 1024];
    let buf = Buf::new(&mut bytes);
}


// Answer 0

#[test]
fn test_put_bytes_len_0_cnt_1() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 0];
    let val: u8 = 42;
    let cnt: usize = 1;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_len_1_cnt_2() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 1];
    let val: u8 = 42;
    let cnt: usize = 2;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_len_5_cnt_10() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let val: u8 = 42;
    let cnt: usize = 10;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_len_10_cnt_20() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 10];
    let val: u8 = 42;
    let cnt: usize = 20;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_len_99_cnt_100() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 99];
    let val: u8 = 42;
    let cnt: usize = 100;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_len_100_cnt_200() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 100];
    let val: u8 = 42;
    let cnt: usize = 200;
    unsafe {
        buffer.put_bytes(val, cnt);
    }
}


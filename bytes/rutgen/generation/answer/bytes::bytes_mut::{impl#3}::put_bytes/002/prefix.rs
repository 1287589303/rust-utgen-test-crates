// Answer 0

#[test]
fn test_put_bytes_with_cnt_greater_than_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let val: u8 = 127; 
    let cnt: usize = 11; // cnt is greater than the current capacity
    bytes_mut.put_bytes(val, cnt);
}

#[test]
fn test_put_bytes_with_cnt_equal_to_capacity_plus_one() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    let val: u8 = 200; 
    let cnt: usize = 6; // cnt is equal to capacity + 1
    bytes_mut.put_bytes(val, cnt);
}

#[test]
fn test_put_bytes_with_zero_length() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    let val: u8 = 255; 
    let cnt: usize = 0; // cnt is zero
    bytes_mut.put_bytes(val, cnt);
}


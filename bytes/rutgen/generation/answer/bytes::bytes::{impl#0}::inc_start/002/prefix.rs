// Answer 0

#[test]
#[should_panic]
fn test_inc_start_with_len_zero() {
    let mut bytes = Bytes::new(); // self.len is 0
    unsafe { bytes.inc_start(1); } // by is greater than self.len
}

#[test]
#[should_panic]
fn test_inc_start_with_len_five() {
    let mut bytes = Bytes::from_static(b"Hello"); // self.len is 5
    unsafe { bytes.inc_start(6); } // by is greater than self.len
}

#[test]
#[should_panic]
fn test_inc_start_with_len_three() {
    let bytes = Bytes::from_static(b"ABC"); // self.len is 3
    unsafe { bytes.inc_start(5); } // by is greater than self.len
}


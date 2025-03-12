// Answer 0

#[test]
#[should_panic]
fn test_put_slice_panic_case_one() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    let src: &[u8] = &[1, 2, 3, 4, 5]; // src.len() > buf.len()
    buf.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_panic_case_two() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 1];
    let src: &[u8] = &[6, 7]; // src.len() > buf.len()
    buf.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_panic_case_three() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 0];
    let src: &[u8] = &[8]; // src.len() > buf.len()
    buf.put_slice(src);
}


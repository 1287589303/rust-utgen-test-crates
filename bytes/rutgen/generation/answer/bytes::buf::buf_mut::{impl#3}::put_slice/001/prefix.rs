// Answer 0

#[test]
#[should_panic]
fn test_put_slice_panic_case_1() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    let src: &[u8] = &[1, 2, 3, 4, 5]; // src.len() = 5, self.len() = 3
    buffer.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_panic_case_2() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 0];
    let src: &[u8] = &[1]; // src.len() = 1, self.len() = 0
    buffer.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_panic_case_3() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 10];
    let src: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // src.len() = 11, self.len() = 10
    buffer.put_slice(src);
}


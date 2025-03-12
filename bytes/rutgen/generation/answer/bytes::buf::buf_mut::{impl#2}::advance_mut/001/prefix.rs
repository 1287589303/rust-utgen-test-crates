// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panic_case_1() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [];
    unsafe { buffer.advance_mut(1) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_case_2() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit()];
    unsafe { buffer.advance_mut(2) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_case_3() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    unsafe { buffer.advance_mut(4) };
}


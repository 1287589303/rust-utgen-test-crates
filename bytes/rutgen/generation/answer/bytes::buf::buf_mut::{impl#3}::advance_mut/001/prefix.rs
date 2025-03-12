// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panic_case_1() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let cnt = 10; // cnt is greater than buf.len()
    unsafe { buf.advance_mut(cnt) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_case_2() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 0];
    let cnt = 1; // cnt is greater than buf.len()
    unsafe { buf.advance_mut(cnt) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_case_3() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    let cnt = 5; // cnt is greater than buf.len()
    unsafe { buf.advance_mut(cnt) };
}


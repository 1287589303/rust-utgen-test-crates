// Answer 0

#[test]
fn test_advance_mut_zero_length() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [];
    unsafe { buf.advance_mut(0) };
}

#[test]
fn test_advance_mut_one_length() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit()];
    unsafe { buf.advance_mut(1) };
}

#[test]
fn test_advance_mut_two_length() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit()];
    unsafe { buf.advance_mut(2) };
}

#[test]
fn test_advance_mut_three_length() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit()];
    unsafe { buf.advance_mut(3) };
}

#[test]
fn test_advance_mut_four_length() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit(), core::mem::MaybeUninit::uninit()];
    unsafe { buf.advance_mut(4) };
}


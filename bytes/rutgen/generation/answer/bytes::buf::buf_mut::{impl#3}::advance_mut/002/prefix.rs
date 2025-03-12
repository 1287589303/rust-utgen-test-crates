// Answer 0

#[test]
fn test_advance_mut_equal_length() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [
        core::mem::MaybeUninit::new(0), 
        core::mem::MaybeUninit::new(1)
    ];
    let cnt = buffer.len();
    unsafe { buffer.advance_mut(cnt) };
}

#[test]
fn test_advance_mut_single_element() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [
        core::mem::MaybeUninit::new(42)
    ];
    let cnt = buffer.len();
    unsafe { buffer.advance_mut(cnt) };
}


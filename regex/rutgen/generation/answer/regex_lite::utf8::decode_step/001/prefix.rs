// Answer 0

#[test]
fn test_decode_step_accept_state_with_minimum_byte() {
    let mut state: usize = 12;
    let mut cp: u32 = 0;
    let b: u8 = 0;
    decode_step(&mut state, &mut cp, b);
}

#[test]
fn test_decode_step_accept_state_with_middle_byte() {
    let mut state: usize = 12;
    let mut cp: u32 = 0;
    let b: u8 = 128;
    decode_step(&mut state, &mut cp, b);
}

#[test]
fn test_decode_step_accept_state_with_maximum_byte() {
    let mut state: usize = 12;
    let mut cp: u32 = 0;
    let b: u8 = 255;
    decode_step(&mut state, &mut cp, b);
}


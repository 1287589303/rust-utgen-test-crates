// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_capacity_zero() {
    let mut buffer: Vec<u8> = Vec::with_capacity(0);
    unsafe { buffer.advance_mut(1) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_capacity_one() {
    let mut buffer: Vec<u8> = Vec::with_capacity(1);
    unsafe { buffer.advance_mut(2) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_capacity_two() {
    let mut buffer: Vec<u8> = Vec::with_capacity(2);
    unsafe { buffer.advance_mut(3) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_capacity_three() {
    let mut buffer: Vec<u8> = Vec::with_capacity(3);
    unsafe { buffer.advance_mut(4) };
}

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_capacity_five() {
    let mut buffer: Vec<u8> = Vec::with_capacity(5);
    unsafe { buffer.advance_mut(6) };
}


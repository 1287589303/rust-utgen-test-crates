// Answer 0

#[test]
#[should_panic]
fn test_fmt_write_error_empty_slots() {
    let slots = Slots(0);
    let mut f = core::fmt::Formatter::new();
    slots.fmt(&mut f).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_write_error_full_slots() {
    let slots = Slots(u32::MAX);
    let mut f = core::fmt::Formatter::new();
    slots.fmt(&mut f).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_write_error_single_slot() {
    let slots = Slots(1);
    let mut f = core::fmt::Formatter::new();
    slots.fmt(&mut f).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_write_error_multiple_slots() {
    let slots = Slots(0b00001111);
    let mut f = core::fmt::Formatter::new();
    slots.fmt(&mut f).unwrap();
}


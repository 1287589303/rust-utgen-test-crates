// Answer 0

#[test]
fn test_remove_slot_0() {
    let slots = Slots(0b11111111111111111111111111111111);
    let result = slots.remove(0);
}

#[test]
fn test_remove_slot_15() {
    let slots = Slots(0b11111111111111111111111111111111);
    let result = slots.remove(15);
}

#[test]
fn test_remove_slot_31() {
    let slots = Slots(0b11111111111111111111111111111111);
    let result = slots.remove(31);
}

#[test]
fn test_remove_slot_boundary_below_0() {
    let slots = Slots(0b11111111111111111111111111111111);
    let result = slots.remove(0);
}

#[test]
fn test_remove_slot_boundary_above_31() {
    let slots = Slots(0b11111111111111111111111111111111);
    let result = slots.remove(31);
}


// Answer 0

#[test]
fn test_insert_slot_zero() {
    let slots = Slots(0);
    let result = slots.insert(0);
}

#[test]
fn test_insert_slot_one() {
    let slots = Slots(0);
    let result = slots.insert(1);
}

#[test]
fn test_insert_slot_two() {
    let slots = Slots(0);
    let result = slots.insert(2);
}

#[test]
fn test_insert_slot_thirty_one() {
    let slots = Slots(0);
    let result = slots.insert(31);
}

#[test]
fn test_insert_slot_mid_range() {
    let slots = Slots(0);
    let result = slots.insert(15);
}


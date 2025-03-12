// Answer 0

#[test]
#[should_panic]
fn test_insert_slot_equal_limit() {
    let slots = Slots(0);
    let _ = slots.insert(Slots::LIMIT);
}

#[test]
fn test_insert_slot_zero() {
    let slots = Slots(0);
    let _ = slots.insert(0);
}

#[test]
fn test_insert_slot_one() {
    let slots = Slots(0);
    let _ = slots.insert(1);
}

#[test]
fn test_insert_slot_two() {
    let slots = Slots(0);
    let _ = slots.insert(2);
}

#[test]
fn test_insert_slot_thirty_one() {
    let slots = Slots(0);
    let _ = slots.insert(31);
}


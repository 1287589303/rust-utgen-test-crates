// Answer 0

#[test]
fn test_apply_with_empty_slots() {
    let slots = Slots(1); // non-empty
    let at = 0; // valid at
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![None; 1]; // length is 1
    slots.apply(at, &mut caller_explicit_slots);
}

#[test]
fn test_apply_with_full_slots() {
    let slots = Slots(0b11111111111111111111111111111111); // non-empty and all slots set
    let at = 15; // valid at
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![None; 32]; // length is 32
    slots.apply(at, &mut caller_explicit_slots);
}

#[test]
fn test_apply_with_just_out_of_bounds() {
    let slots = Slots(2); // non-empty, sets slot 1
    let at = 8; // valid at
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![None; 2]; // length is 2
    slots.apply(at, &mut caller_explicit_slots);
}

#[test]
fn test_apply_with_slots_equal_to_length() {
    let slots = Slots(4); // non-empty, sets slot 2
    let at = 10; // valid at
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![None; 4]; // length is 4
    slots.apply(at, &mut caller_explicit_slots);
}


// Answer 0

#[test]
fn test_apply_empty_slots() {
    let slots = Slots(0); // self.is_empty() is true
    let at = 5; // any valid usize
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![None; 10]; // slice of length 10
    slots.apply(at, &mut caller_explicit_slots);
}

#[test]
fn test_apply_empty_slots_zero_length() {
    let slots = Slots(0); // self.is_empty() is true
    let at = 1; // any valid usize
    let mut caller_explicit_slots: Vec<Option<NonMaxUsize>> = vec![]; // zero length slice
    slots.apply(at, &mut caller_explicit_slots);
}


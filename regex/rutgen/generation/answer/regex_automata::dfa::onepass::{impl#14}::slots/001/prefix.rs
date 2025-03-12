// Answer 0

#[test]
fn test_slots_empty() {
    let epsilons = Epsilons(0);
    let _ = epsilons.slots();
}

#[test]
fn test_slots_max_value() {
    let epsilons = Epsilons(u64::MAX);
    let _ = epsilons.slots();
}

#[test]
fn test_slots_at_slot_shift_boundary() {
    let epsilons = Epsilons((1 << Epsilons::SLOT_SHIFT) - 1);
    let _ = epsilons.slots();
}


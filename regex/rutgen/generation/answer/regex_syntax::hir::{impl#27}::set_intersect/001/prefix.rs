// Answer 0

#[test]
fn test_set_intersect_empty_self() {
    let mut self_set = LookSet::empty();
    let other_set = LookSet::singleton(Look::SomeVariant);
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_empty_other() {
    let mut self_set = LookSet::singleton(Look::SomeVariant);
    let other_set = LookSet::empty();
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_full() {
    let mut self_set = LookSet::full();
    let other_set = LookSet::full();
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_partial() {
    let mut self_set = LookSet::singleton(Look::SomeVariant);
    let other_set = LookSet::singleton(Look::AnotherVariant);
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_same_variant() {
    let mut self_set = LookSet::singleton(Look::SomeVariant);
    let other_set = LookSet::singleton(Look::SomeVariant);
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_multiple_bits() {
    let mut self_set = LookSet { bits: 0b1100 };
    let other_set = LookSet { bits: 0b1010 };
    self_set.set_intersect(other_set);
}

#[test]
fn test_set_intersect_no_overlap() {
    let mut self_set = LookSet { bits: 0b0011 };
    let other_set = LookSet { bits: 0b1100 };
    self_set.set_intersect(other_set);
}


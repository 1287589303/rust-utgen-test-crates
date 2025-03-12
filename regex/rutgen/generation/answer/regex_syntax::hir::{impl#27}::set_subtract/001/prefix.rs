// Answer 0

#[test]
fn test_set_subtract_empty_from_non_empty() {
    let mut set_a = LookSet { bits: 0xFFFFFFFF };
    let set_b = LookSet::empty();
    set_a.set_subtract(set_b);
}

#[test]
fn test_set_subtract_non_empty_from_empty() {
    let mut set_a = LookSet::empty();
    let set_b = LookSet { bits: 0xFFFFFFFF };
    set_a.set_subtract(set_b);
}

#[test]
fn test_set_subtract_equal_sets() {
    let mut set_a = LookSet { bits: 0x12345678 };
    let set_b = LookSet { bits: 0x12345678 };
    set_a.set_subtract(set_b);
}

#[test]
fn test_set_subtract_non_empty_partial() {
    let mut set_a = LookSet { bits: 0xFFFFFFFF };
    let set_b = LookSet { bits: 0x0F0F0F0F };
    set_a.set_subtract(set_b);
}

#[test]
fn test_set_subtract_with_no_overlap() {
    let mut set_a = LookSet { bits: 0xF0F0F0F0 };
    let set_b = LookSet { bits: 0x0F0F0F0F };
    set_a.set_subtract(set_b);
}

#[test]
fn test_set_subtract_full_set() {
    let mut set_a = LookSet { bits: 0xFFFFFFFF };
    let set_b = LookSet::full();
    set_a.set_subtract(set_b);
}


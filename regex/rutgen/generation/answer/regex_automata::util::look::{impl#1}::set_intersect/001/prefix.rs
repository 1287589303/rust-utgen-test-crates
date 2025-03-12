// Answer 0

#[test]
fn test_set_intersect_with_empty() {
    let mut set_a = LookSet::full();
    let set_b = LookSet::empty();
    set_a.set_intersect(set_b);
}

#[test]
fn test_set_intersect_with_full() {
    let mut set_a = LookSet::empty();
    let set_b = LookSet::full();
    set_a.set_intersect(set_b);
}

#[test]
fn test_set_intersect_with_partial_overlap() {
    let mut set_a = LookSet { bits: 0b1100 };
    let set_b = LookSet { bits: 0b1010 };
    set_a.set_intersect(set_b);
}

#[test]
fn test_set_intersect_with_no_overlap() {
    let mut set_a = LookSet { bits: 0b0001 };
    let set_b = LookSet { bits: 0b1110 };
    set_a.set_intersect(set_b);
}

#[test]
fn test_set_intersect_with_identical() {
    let mut set_a = LookSet { bits: 0b1111 };
    let set_b = LookSet { bits: 0b1111 };
    set_a.set_intersect(set_b);
}

#[test]
fn test_set_intersect_with_different_size() {
    let mut set_a = LookSet { bits: 0b1010 };
    let set_b = LookSet { bits: 0b0101 }; 
    set_a.set_intersect(set_b);
}


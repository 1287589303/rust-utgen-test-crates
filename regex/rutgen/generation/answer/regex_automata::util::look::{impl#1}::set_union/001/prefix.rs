// Answer 0

#[test]
fn test_set_union_empty_sets() {
    let mut set1 = LookSet::empty();
    let set2 = LookSet::empty();
    set1.set_union(set2);
}

#[test]
fn test_set_union_set1_empty_set2_full() {
    let mut set1 = LookSet::empty();
    let set2 = LookSet::full();
    set1.set_union(set2);
}

#[test]
fn test_set_union_set1_full_set2_empty() {
    let mut set1 = LookSet::full();
    let set2 = LookSet::empty();
    set1.set_union(set2);
}

#[test]
fn test_set_union_partially_filled_sets() {
    let mut set1 = LookSet { bits: 0b00000011 }; // two bits set
    let set2 = LookSet { bits: 0b00000101 }; // two bits set
    set1.set_union(set2);
}

#[test]
fn test_set_union_full_sets() {
    let mut set1 = LookSet::full();
    let set2 = LookSet::full();
    set1.set_union(set2);
}

#[test]
fn test_set_union_partial_and_full() {
    let mut set1 = LookSet { bits: 0b00000011 };
    let set2 = LookSet::full();
    set1.set_union(set2);
}

#[test]
fn test_set_union_identical_sets() {
    let mut set1 = LookSet { bits: 0b00000011 };
    let set2 = LookSet { bits: 0b00000011 };
    set1.set_union(set2);
}


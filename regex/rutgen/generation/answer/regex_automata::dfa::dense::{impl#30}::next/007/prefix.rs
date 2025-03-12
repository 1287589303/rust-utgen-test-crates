// Answer 0

#[test]
fn test_next_with_eoi_and_dead() {
    let state_id = StateID(Default::default());
    let unit_eoi = Unit::eoi(1);
    let unit_next = unit_eoi;
    let state_ids = vec![state_id];

    let mut dense_iter = StateTransitionIter {
        len: state_ids.len(),
        it: state_ids.iter().enumerate(),
    };

    let mut iter = StateSparseTransitionIter {
        dense: dense_iter,
        cur: Some((unit_eoi, unit_eoi, state_id)),
    };

    let result = iter.next();
}

#[test]
fn test_next_with_eoi_and_non_dead() {
    let state_id = StateID(Default::default());
    let unit_eoi = Unit::eoi(1);
    let unit_next = unit_eoi;
    let state_ids = vec![state_id];

    let mut dense_iter = StateTransitionIter {
        len: state_ids.len(),
        it: state_ids.iter().enumerate(),
    };

    let mut iter = StateSparseTransitionIter {
        dense: dense_iter,
        cur: Some((unit_eoi, unit_eoi, state_id)),
    };

    let result = iter.next();
}

#[test]
fn test_next_with_non_dead() {
    let state_id = StateID(Default::default());
    let unit_a = Unit::u8(0);
    let unit_b = Unit::u8(1);
    let state_ids = vec![state_id];

    let mut dense_iter = StateTransitionIter {
        len: state_ids.len(),
        it: state_ids.iter().enumerate(),
    };

    let mut iter = StateSparseTransitionIter {
        dense: dense_iter,
        cur: Some((unit_a, unit_b, state_id)),
    };

    let result = iter.next();
}


// Answer 0

#[test]
fn test_next_with_valid_inputs() {
    let unit1 = alphabet::Unit::u8(65); // 'A'
    let next1 = StateID(1);
    let unit2 = alphabet::Unit::u8(66); // 'B'
    let next2 = StateID(2);
   
    let transitions = vec![next1, next2];
    let dense = StateTransitionIter {
        len: transitions.len(),
        it: transitions.iter().enumerate(),
    };
    
    let mut iter = StateSparseTransitionIter {
        dense,
        cur: None,
    };

    if let Some((unit, next)) = iter.dense.next() {
        iter.cur = Some((unit, unit, next));
    }

    if let Some((start, end, next)) = iter.next() {
        let _ = (start, end, next); // Simulate return
    }
}

#[test]
fn test_next_with_eoi_and_valid_next() {
    let unit1 = alphabet::Unit::u8(67); // 'C'
    let next1 = StateID(3);
    
    let transitions = vec![next1];
    let dense = StateTransitionIter {
        len: transitions.len(),
        it: transitions.iter().enumerate(),
    };
    
    let mut iter = StateSparseTransitionIter {
        dense,
        cur: None,
    };

    if let Some((unit, next)) = iter.dense.next() {
        iter.cur = Some((unit, unit, next));
    }

    if let Some((start, end, next)) = iter.next() {
        let _ = (start, end, next); // Simulate return
    }
}


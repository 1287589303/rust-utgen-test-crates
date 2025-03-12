// Answer 0

#[test]
fn test_add_nfa_state_id_positive_delta() {
    let mut vec = vec![0u8; 10];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(1));
    let sid = StateID(SmallIndex(5));
    repr.add_nfa_state_id(&mut prev, sid);
}

#[test]
fn test_add_nfa_state_id_negative_delta() {
    let mut vec = vec![0u8; 10];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(5));
    let sid = StateID(SmallIndex(1));
    repr.add_nfa_state_id(&mut prev, sid);
}

#[test]
fn test_add_nfa_state_id_large_positive_delta() {
    let mut vec = vec![0u8; 20];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(100));
    let sid = StateID(SmallIndex(200));
    repr.add_nfa_state_id(&mut prev, sid);
}

#[test]
fn test_add_nfa_state_id_large_negative_delta() {
    let mut vec = vec![0u8; 20];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(200));
    let sid = StateID(SmallIndex(100));
    repr.add_nfa_state_id(&mut prev, sid);
}

#[test]
fn test_add_nfa_state_id_with_minimum_negative_delta() {
    let mut vec = vec![0u8; 10];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(0));
    let sid = StateID(SmallIndex(1));
    repr.add_nfa_state_id(&mut prev, sid);
}

#[test]
fn test_add_nfa_state_id_with_maximum_positive_delta() {
    let mut vec = vec![0u8; 10];
    let mut repr = ReprVec(&mut vec);
    let mut prev = StateID(SmallIndex(i32::MAX as usize - 1));
    let sid = StateID(SmallIndex(i32::MAX as usize));
    repr.add_nfa_state_id(&mut prev, sid);
}


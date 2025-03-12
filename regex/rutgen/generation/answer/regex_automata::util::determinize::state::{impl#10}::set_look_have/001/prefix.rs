// Answer 0

#[test]
fn test_set_look_have_empty() {
    let mut vec = vec![0u8; 4];
    let mut repr_vec = ReprVec(&mut vec);
    repr_vec.set_look_have(|_| LookSet::empty());
}

#[test]
fn test_set_look_have_full() {
    let mut vec = vec![0u8; 4];
    let mut repr_vec = ReprVec(&mut vec);
    repr_vec.set_look_have(|_| LookSet::full());
}

#[test]
fn test_set_look_have_singleton() {
    let mut vec = vec![0u8; 4];
    let mut repr_vec = ReprVec(&mut vec);
    repr_vec.set_look_have(|_| LookSet::singleton(Look::from_repr(0)));
}

#[test]
fn test_set_look_have_minimal() {
    let mut vec = vec![0u8; 4];
    let mut repr_vec = ReprVec(&mut vec);
    let minimal_lookset = LookSet { bits: 0 };
    repr_vec.set_look_have(|_| minimal_lookset);
}

#[test]
fn test_set_look_have_maximal() {
    let mut vec = vec![0u8; 4];
    let mut repr_vec = ReprVec(&mut vec);
    let maximal_lookset = LookSet { bits: u32::MAX };
    repr_vec.set_look_have(|_| maximal_lookset);
}


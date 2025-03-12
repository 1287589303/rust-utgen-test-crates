// Answer 0

#[test]
fn test_explicit_slots_empty() {
    let mut cache = Cache {
        explicit_slots: vec![],
        explicit_slot_len: 0,
    };
    let slots = cache.explicit_slots();
}

#[test]
fn test_explicit_slots_one() {
    let non_max_usize = NonMaxUsize(NonZeroUsize::new(1).unwrap());
    let mut cache = Cache {
        explicit_slots: vec![Some(non_max_usize)],
        explicit_slot_len: 1,
    };
    let slots = cache.explicit_slots();
}

#[test]
fn test_explicit_slots_multiple() {
    let non_max_usize1 = NonMaxUsize(NonZeroUsize::new(1).unwrap());
    let non_max_usize2 = NonMaxUsize(NonZeroUsize::new(2).unwrap());
    let mut cache = Cache {
        explicit_slots: vec![Some(non_max_usize1), Some(non_max_usize2)],
        explicit_slot_len: 2,
    };
    let slots = cache.explicit_slots();
}

#[test]
fn test_explicit_slots_full_length() {
    let non_max_usize1 = NonMaxUsize(NonZeroUsize::new(1).unwrap());
    let non_max_usize2 = NonMaxUsize(NonZeroUsize::new(2).unwrap());
    let non_max_usize3 = NonMaxUsize(NonZeroUsize::new(3).unwrap());
    let mut cache = Cache {
        explicit_slots: vec![Some(non_max_usize1), Some(non_max_usize2), Some(non_max_usize3)],
        explicit_slot_len: 3,
    };
    let slots = cache.explicit_slots();
}

#[test]
fn test_explicit_slots_boundary() {
    let mut cache = Cache {
        explicit_slots: vec![None; 10],
        explicit_slot_len: 10,
    };
    let slots = cache.explicit_slots();
}


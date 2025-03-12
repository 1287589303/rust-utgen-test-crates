// Answer 0

#[test]
fn test_contains_with_existing_id() {
    #[cfg(feature = "alloc")]
    {
        let mut seen = Seen {
            set: alloc::collections::BTreeSet::new(),
        };
        let state_id = StateID(SmallIndex::new(1));
        seen.insert(state_id);
        
        let result = seen.contains(&state_id);
    }
}

#[test]
fn test_contains_with_non_existing_id() {
    #[cfg(feature = "alloc")]
    {
        let mut seen = Seen {
            set: alloc::collections::BTreeSet::new(),
        };
        let existing_id = StateID(SmallIndex::new(1));
        seen.insert(existing_id);
        let non_existing_id = StateID(SmallIndex::new(2));
        
        let result = seen.contains(&non_existing_id);
    }
}

#[test]
fn test_contains_with_default_id() {
    #[cfg(feature = "alloc")]
    {
        let mut seen = Seen {
            set: alloc::collections::BTreeSet::new(),
        };
        let default_id = StateID(SmallIndex::default());
        
        let result = seen.contains(&default_id);
    }
}


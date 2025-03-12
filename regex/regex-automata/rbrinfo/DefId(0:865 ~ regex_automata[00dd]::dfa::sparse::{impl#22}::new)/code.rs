fn new() -> Seen {
        Seen { set: alloc::collections::BTreeSet::new() }
    }
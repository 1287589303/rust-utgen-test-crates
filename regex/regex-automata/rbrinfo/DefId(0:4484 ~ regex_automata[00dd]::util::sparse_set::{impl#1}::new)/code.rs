pub(crate) fn new(capacity: usize) -> SparseSet {
        let mut set = SparseSet { len: 0, dense: vec![], sparse: vec![] };
        set.resize(capacity);
        set
    }
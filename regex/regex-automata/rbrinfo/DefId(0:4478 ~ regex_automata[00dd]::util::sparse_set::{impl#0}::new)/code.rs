pub(crate) fn new(capacity: usize) -> SparseSets {
        SparseSets {
            set1: SparseSet::new(capacity),
            set2: SparseSet::new(capacity),
        }
    }
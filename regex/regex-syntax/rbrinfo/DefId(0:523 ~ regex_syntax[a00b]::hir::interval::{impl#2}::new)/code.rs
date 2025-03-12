pub fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I> {
        let ranges: Vec<I> = intervals.into_iter().collect();
        // An empty set is case folded.
        let folded = ranges.is_empty();
        let mut set = IntervalSet { ranges, folded };
        set.canonicalize();
        set
    }
fn new<I: IntoIterator<Item = ClassRange>>(ranges: I) -> Class {
        let mut class = Class { ranges: ranges.into_iter().collect() };
        class.canonicalize();
        class
    }
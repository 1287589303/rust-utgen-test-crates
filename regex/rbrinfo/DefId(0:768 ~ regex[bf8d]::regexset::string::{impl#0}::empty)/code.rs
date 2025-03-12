pub fn empty() -> RegexSet {
        let empty: [&str; 0] = [];
        RegexSetBuilder::new(empty).build().unwrap()
    }
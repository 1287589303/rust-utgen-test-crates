pub fn new(re: &BoundedBacktracker) -> Cache {
        Cache { stack: vec![], visited: Visited::new(re) }
    }
fn new(re: &BoundedBacktracker) -> Visited {
        let mut visited = Visited { bitset: vec![], stride: 0 };
        visited.reset(re);
        visited
    }
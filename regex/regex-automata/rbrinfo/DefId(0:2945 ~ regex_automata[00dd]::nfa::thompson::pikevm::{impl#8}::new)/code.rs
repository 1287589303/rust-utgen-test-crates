pub fn new(re: &PikeVM) -> Cache {
        Cache {
            stack: vec![],
            curr: ActiveStates::new(re),
            next: ActiveStates::new(re),
        }
    }
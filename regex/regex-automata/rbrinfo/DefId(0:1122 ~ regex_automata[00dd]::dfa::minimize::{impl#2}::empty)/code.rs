fn empty() -> StateSet {
        StateSet { ids: Rc::new(RefCell::new(vec![])) }
    }
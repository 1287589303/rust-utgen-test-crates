// Answer 0

#[test]
fn test_is_empty_with_empty_stateset() {
    let state_set = StateSet {
        ids: Rc::new(RefCell::new(Vec::new())),
    };
    state_set.is_empty();
}

#[test]
fn test_is_empty_with_non_empty_stateset() {
    let state_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(0), StateID(1)])),
    };
    state_set.is_empty();
}


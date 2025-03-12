// Answer 0

#[test]
fn test_clear_non_empty_stateset() {
    let mut stateset = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(SmallIndex::new(1)), StateID(SmallIndex::new(2))])),
    };
    stateset.clear();
}

#[test]
fn test_clear_empty_stateset() {
    let mut stateset = StateSet {
        ids: Rc::new(RefCell::new(vec![])),
    };
    stateset.clear();
}


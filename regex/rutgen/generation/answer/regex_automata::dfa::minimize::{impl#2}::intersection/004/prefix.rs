// Answer 0

#[test]
fn test_intersection_common_elements() {
    let mut self_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(1), StateID(2)])),
    };
    let other_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(2), StateID(3)])),
    };
    let mut dest_set = StateSet::empty();
    
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_no_common_elements() {
    let mut self_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(1), StateID(2)])),
    };
    let other_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(3), StateID(4)])),
    };
    let mut dest_set = StateSet::empty();
    
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_repeated_common_elements() {
    let mut self_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(1), StateID(2), StateID(2)])),
    };
    let other_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(2), StateID(2), StateID(3)])),
    };
    let mut dest_set = StateSet::empty();
    
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_empty_dest() {
    let mut self_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(1), StateID(3)])),
    };
    let other_set = StateSet {
        ids: Rc::new(RefCell::new(vec![StateID(3), StateID(4)])),
    };
    let mut dest_set = StateSet::empty();
    
    self_set.intersection(&other_set, &mut dest_set);
}


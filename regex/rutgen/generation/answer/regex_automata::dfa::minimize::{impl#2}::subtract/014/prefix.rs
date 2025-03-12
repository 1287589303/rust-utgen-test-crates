// Answer 0

#[test]
fn test_subtract_a_greater_than_b() {
    let mut self_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(1), StateID(3), StateID(5)])) };
    let other_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(2), StateID(4)])) };
    let mut dest_set = StateSet::empty();

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_a_equals_b() {
    let mut self_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(2), StateID(4)])) };
    let other_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(2), StateID(4)])) };
    let mut dest_set = StateSet::empty();

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_a_less_than_b_continue_after_equal() {
    let mut self_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(1), StateID(3), StateID(5)])) };
    let other_set = StateSet { ids: Rc::new(RefCell::new(vec![StateID(1), StateID(4)])) };
    let mut dest_set = StateSet::empty();

    self_set.subtract(&other_set, &mut dest_set);
}


// Answer 0

#[test]
fn test_is_subset_with_greater_length() {
    let mut self_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    self_set.insert("a");
    self_set.insert("b");
    self_set.insert("c");
    self_set.insert("d");
    self_set.insert("e");

    let mut other_set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    other_set.insert("a");
    other_set.insert("b");
    other_set.insert("c");

    let result = self_set.is_subset(&other_set);
}

#[test]
fn test_is_subset_with_missing_element() {
    let mut self_set = IndexSet::with_capacity_and_hasher(4, RandomState::new());
    self_set.insert("x");
    self_set.insert("y");
    self_set.insert("z");
    self_set.insert("w");

    let mut other_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    other_set.insert("x");
    other_set.insert("y");
    other_set.insert("a");
    other_set.insert("b");
    other_set.insert("c");

    let result = self_set.is_subset(&other_set);
}

#[test]
fn test_is_subset_with_disjoint(self_set: &mut IndexSet<&str, RandomState>) {
    let mut self_set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    self_set.insert("1");
    self_set.insert("2");
    self_set.insert("3");

    let mut other_set = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    other_set.insert("a");
    other_set.insert("b");

    let result = self_set.is_subset(&other_set);
}


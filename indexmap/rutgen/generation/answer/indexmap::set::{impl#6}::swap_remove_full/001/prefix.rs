// Answer 0

#[test]
fn test_swap_remove_full_existing_value() {
    struct MyEquivalent(i32);
    impl Hash for MyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<MyEquivalent> for MyEquivalent {
        fn equivalent(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let mut set = super::IndexSet::<MyEquivalent, std::collections::hash_map::RandomState>::new();
    set.insert(MyEquivalent(1));
    set.insert(MyEquivalent(2));
    set.insert(MyEquivalent(3));

    let result = set.swap_remove_full(&MyEquivalent(2));
}

#[test]
fn test_swap_remove_full_non_existing_value() {
    struct MyEquivalent(i32);
    impl Hash for MyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<MyEquivalent> for MyEquivalent {
        fn equivalent(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let mut set = super::IndexSet::<MyEquivalent, std::collections::hash_map::RandomState>::new();
    set.insert(MyEquivalent(1));
    set.insert(MyEquivalent(2));
    
    let result = set.swap_remove_full(&MyEquivalent(3));
}

#[test]
fn test_swap_remove_full_boundary_case_first_element() {
    struct MyEquivalent(i32);
    impl Hash for MyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<MyEquivalent> for MyEquivalent {
        fn equivalent(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let mut set = super::IndexSet::<MyEquivalent, std::collections::hash_map::RandomState>::new();
    set.insert(MyEquivalent(1));
    set.insert(MyEquivalent(2));
    
    let result = set.swap_remove_full(&MyEquivalent(1));
}

#[test]
fn test_swap_remove_full_boundary_case_last_element() {
    struct MyEquivalent(i32);
    impl Hash for MyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<MyEquivalent> for MyEquivalent {
        fn equivalent(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let mut set = super::IndexSet::<MyEquivalent, std::collections::hash_map::RandomState>::new();
    set.insert(MyEquivalent(1));
    set.insert(MyEquivalent(2));
    
    let result = set.swap_remove_full(&MyEquivalent(2));
}


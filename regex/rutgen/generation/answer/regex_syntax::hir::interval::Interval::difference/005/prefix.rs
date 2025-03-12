// Answer 0

#[test]
fn test_difference_case1() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower_bound: i32,
        upper_bound: i32,
    }

    impl TestInterval {
        fn lower(&self) -> i32 { self.lower_bound }
        fn upper(&self) -> i32 { self.upper_bound }
        fn set_lower(&mut self, bound: i32) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: i32) { self.upper_bound = bound; }
        fn is_contiguous(&self, other: &Self) -> bool { 
            self.upper() >= other.lower() && self.lower() <= other.upper() 
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }
    }

    let mut self_interval = TestInterval { lower_bound: 2, upper_bound: 10 };
    let other_interval = TestInterval { lower_bound: 5, upper_bound: 8 };

    let result = self_interval.difference(&other_interval);
}

#[test]
fn test_difference_case2() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower_bound: i32,
        upper_bound: i32,
    }

    impl TestInterval {
        fn lower(&self) -> i32 { self.lower_bound }
        fn upper(&self) -> i32 { self.upper_bound }
        fn set_lower(&mut self, bound: i32) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: i32) { self.upper_bound = bound; }
        fn is_contiguous(&self, other: &Self) -> bool { 
            self.upper() >= other.lower() && self.lower() <= other.upper() 
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }
    }

    let mut self_interval = TestInterval { lower_bound: 1, upper_bound: 6 };
    let other_interval = TestInterval { lower_bound: 4, upper_bound: 7 };

    let result = self_interval.difference(&other_interval);
}

#[test]
fn test_difference_case3() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower_bound: i32,
        upper_bound: i32,
    }

    impl TestInterval {
        fn lower(&self) -> i32 { self.lower_bound }
        fn upper(&self) -> i32 { self.upper_bound }
        fn set_lower(&mut self, bound: i32) { self.lower_bound = bound; }
        fn set_upper(&mut self, bound: i32) { self.upper_bound = bound; }
        fn is_contiguous(&self, other: &Self) -> bool { 
            self.upper() >= other.lower() && self.lower() <= other.upper() 
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower() >= other.lower() && self.upper() <= other.upper()
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || self.lower() > other.upper()
        }
    }

    let mut self_interval = TestInterval { lower_bound: 3, upper_bound: 9 };
    let other_interval = TestInterval { lower_bound: 6, upper_bound: 10 };

    let result = self_interval.difference(&other_interval);
}


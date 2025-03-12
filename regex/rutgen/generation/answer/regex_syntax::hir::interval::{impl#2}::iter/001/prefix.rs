// Answer 0

#[test]
fn test_iter_empty() {
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            Self { lower: self.lower, upper: self.upper }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            Self { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}
    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }
    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }
    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }
    
    let empty_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    let _iter = empty_set.iter();
}

#[test]
fn test_iter_single_interval() {
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            Self { lower: self.lower, upper: self.upper }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            Self { lower: 1, upper: 1 }
        }
    }

    impl Eq for TestInterval {}
    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }
    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }
    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    let single_interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![TestInterval { lower: 1, upper: 1 }]);
    let _iter = single_interval_set.iter();
}

#[test]
fn test_iter_multiple_intervals() {
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            Self { lower: self.lower, upper: self.upper }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            Self { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}
    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }
    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }
    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    let multiple_intervals_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 2 },
        TestInterval { lower: 3, upper: 4 },
        TestInterval { lower: 5, upper: 6 },
    ]);
    let _iter = multiple_intervals_set.iter();
}

#[test]
fn test_iter_consecutive_intervals() {
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            Self { lower: self.lower, upper: self.upper }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            Self { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}
    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }
    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }
    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    let consecutive_intervals_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 3 },
        TestInterval { lower: 4, upper: 6 },
        TestInterval { lower: 7, upper: 9 },
    ]);
    let _iter = consecutive_intervals_set.iter();
}


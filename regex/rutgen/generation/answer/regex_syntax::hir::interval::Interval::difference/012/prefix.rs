// Answer 0

#[test]
fn test_difference_upper_addition() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyBound(i32);
    
    impl super::Bound for MyBound {}

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyInterval(MyBound, MyBound);

    impl super::Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound {
            self.0
        }

        fn upper(&self) -> Self::Bound {
            self.1
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.1 = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            false
        }

        fn is_subset(&self, other: &Self) -> bool {
            false
        }
    }

    let mut interval_self = MyInterval(MyBound(5), MyBound(10));
    let interval_other = MyInterval(MyBound(15), MyBound(20));
    
    let result = interval_self.difference(&interval_other);
    // result should be (Some(self), None)
} 

#[test]
fn test_difference_lower_owner() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyBound(i32);
    
    impl super::Bound for MyBound {}

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyInterval(MyBound, MyBound);

    impl super::Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound {
            self.0
        }

        fn upper(&self) -> Self::Bound {
            self.1
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.1 = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }
        
        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            false
        }

        fn is_subset(&self, other: &Self) -> bool {
            false
        }
    }

    let mut interval_self = MyInterval(MyBound(5), MyBound(15));
    let interval_other = MyInterval(MyBound(20), MyBound(25));
    
    let result = interval_self.difference(&interval_other);
    // result should be (Some(5, 20), None) hence cover the upper edge
} 

#[test]
fn test_difference_non_contiguous() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyBound(i32);
    
    impl super::Bound for MyBound {}

    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct MyInterval(MyBound, MyBound);

    impl super::Interval for MyInterval {
        type Bound = MyBound;

        fn lower(&self) -> Self::Bound {
            self.0
        }

        fn upper(&self) -> Self::Bound {
            self.1
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.1 = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError> {
            Ok(())
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            false
        }

        fn is_subset(&self, other: &Self) -> bool {
            false
        }
    }

    let mut interval_self = MyInterval(MyBound(10), MyBound(30));
    let interval_other = MyInterval(MyBound(40), MyBound(50));
    
    let result = interval_self.difference(&interval_other);
    // result should be (Some(self), None)
} 


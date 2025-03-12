// Answer 0

#[test]
fn test_round_with_zero_values() {
    struct TestVec(u32);
    impl ArithOps for TestVec {
        fn add(self, rhs: Self) -> Self {
            TestVec(self.0.wrapping_add(rhs.0))
        }
    }
    
    impl BitOps32 for TestVec {
        fn rotate_each_word_right16(self) -> Self {
            TestVec(self.0.rotate_right(16))
        }
        
        fn rotate_each_word_right20(self) -> Self {
            TestVec(self.0.rotate_right(20))
        }
        
        fn rotate_each_word_right24(self) -> Self {
            TestVec(self.0.rotate_right(24))
        }
        
        fn rotate_each_word_right25(self) -> Self {
            TestVec(self.0.rotate_right(25))
        }
    }
    
    let state = State {
        a: TestVec(0),
        b: TestVec(0),
        c: TestVec(0),
        d: TestVec(0),
    };
    let _result = round(state);
}

#[test]
fn test_round_with_maximum_values() {
    struct TestVec(u32);
    impl ArithOps for TestVec {
        fn add(self, rhs: Self) -> Self {
            TestVec(self.0.wrapping_add(rhs.0))
        }
    }
    
    impl BitOps32 for TestVec {
        fn rotate_each_word_right16(self) -> Self {
            TestVec(self.0.rotate_right(16))
        }
        
        fn rotate_each_word_right20(self) -> Self {
            TestVec(self.0.rotate_right(20))
        }
        
        fn rotate_each_word_right24(self) -> Self {
            TestVec(self.0.rotate_right(24))
        }
        
        fn rotate_each_word_right25(self) -> Self {
            TestVec(self.0.rotate_right(25))
        }
    }

    let state = State {
        a: TestVec(u32::MAX),
        b: TestVec(u32::MAX),
        c: TestVec(u32::MAX),
        d: TestVec(u32::MAX),
    };
    let _result = round(state);
}

#[test]
fn test_round_with_negative_values() {
    struct TestVec(i32);
    impl ArithOps for TestVec {
        fn add(self, rhs: Self) -> Self {
            TestVec(self.0.wrapping_add(rhs.0))
        }
    }
    
    impl BitOps32 for TestVec {
        fn rotate_each_word_right16(self) -> Self {
            TestVec(self.0.rotate_right(16))
        }
        
        fn rotate_each_word_right20(self) -> Self {
            TestVec(self.0.rotate_right(20))
        }
        
        fn rotate_each_word_right24(self) -> Self {
            TestVec(self.0.rotate_right(24))
        }
        
        fn rotate_each_word_right25(self) -> Self {
            TestVec(self.0.rotate_right(25))
        }
    }

    let state = State {
        a: TestVec(-1),
        b: TestVec(-1),
        c: TestVec(-1),
        d: TestVec(-1),
    };
    let _result = round(state);
}

#[test]
fn test_round_with_mixed_values() {
    struct TestVec(i32);
    impl ArithOps for TestVec {
        fn add(self, rhs: Self) -> Self {
            TestVec(self.0.wrapping_add(rhs.0))
        }
    }
    
    impl BitOps32 for TestVec {
        fn rotate_each_word_right16(self) -> Self {
            TestVec(self.0.rotate_right(16))
        }
        
        fn rotate_each_word_right20(self) -> Self {
            TestVec(self.0.rotate_right(20))
        }
        
        fn rotate_each_word_right24(self) -> Self {
            TestVec(self.0.rotate_right(24))
        }
        
        fn rotate_each_word_right25(self) -> Self {
            TestVec(self.0.rotate_right(25))
        }
    }

    let state = State {
        a: TestVec(15),
        b: TestVec(27),
        c: TestVec(-5),
        d: TestVec(0),
    };
    let _result = round(state);
}


// Answer 0

#[test]
fn test_clone_from_both_left() {
    struct LeftType;
    impl Clone for LeftType {
        fn clone(&self) -> Self {
            LeftType
        }
    }

    let mut dest = Left(LeftType);
    let source = Left(LeftType);
    dest.clone_from(&source);
}

#[test]
fn test_clone_from_both_right() {
    struct RightType;
    impl Clone for RightType {
        fn clone(&self) -> Self {
            RightType
        }
    }

    let mut dest = Right(RightType);
    let source = Right(RightType);
    dest.clone_from(&source);
}

#[test]
fn test_clone_from_left_to_right() {
    struct LeftType;
    struct RightType;
    
    impl Clone for LeftType {
        fn clone(&self) -> Self {
            LeftType
        }
    }
    
    let mut dest = Right(RightType);
    let source = Left(LeftType);
    dest.clone_from(&source);
}

#[test]
fn test_clone_from_right_to_left() {
    struct LeftType;
    struct RightType;
    
    impl Clone for RightType {
        fn clone(&self) -> Self {
            RightType
        }
    }
    
    let mut dest = Left(LeftType);
    let source = Right(RightType);
    dest.clone_from(&source);
}


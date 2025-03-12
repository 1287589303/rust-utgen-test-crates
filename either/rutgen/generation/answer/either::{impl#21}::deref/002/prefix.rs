// Answer 0

#[test]
fn test_deref_left() {
    struct LeftWrapper<'a>(&'a str);
    impl Deref for LeftWrapper<'_> {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    let left = Either::Left(LeftWrapper("Hello, Deref!"));
    let _result: &str = &*left;
}

#[test]
fn test_deref_right() {
    struct RightWrapper<'a>(&'a str);
    impl Deref for RightWrapper<'_> {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let right = Either::Right(RightWrapper("Hello, Deref!"));
    let _result: &str = &*right;
}


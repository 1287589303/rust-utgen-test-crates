// Answer 0

#[test]
fn test_start_with_valid_visitor() {
    struct ValidVisitor;

    impl Visitor for ValidVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = ValidVisitor;
    visitor.start();
}

#[test]
fn test_start_with_another_valid_visitor() {
    struct AnotherVisitor;

    impl Visitor for AnotherVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = AnotherVisitor;
    visitor.start();
}


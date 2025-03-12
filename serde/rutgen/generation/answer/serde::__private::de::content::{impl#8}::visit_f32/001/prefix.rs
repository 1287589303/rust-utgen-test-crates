// Answer 0

#[test]
fn test_visit_f32_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(0.0f32);
}

#[test]
fn test_visit_f32_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(1.0f32);
}

#[test]
fn test_visit_f32_negative_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(-1.0f32);
}

#[test]
fn test_visit_f32_large() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(1.0e10f32);
}

#[test]
fn test_visit_f32_small() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(1.0e-10f32);
}

#[test]
#[should_panic]
fn test_visit_f32_nan() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(f32::NAN);
}

#[test]
#[should_panic]
fn test_visit_f32_infinity() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(f32::INFINITY);
}

#[test]
#[should_panic]
fn test_visit_f32_neg_infinity() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    let _ = TestVisitor.visit_f32(f32::NEG_INFINITY);
}


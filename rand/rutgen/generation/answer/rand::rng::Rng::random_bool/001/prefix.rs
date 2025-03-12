// Answer 0

#[test]
#[should_panic]
fn test_random_bool_with_negative_probability() {
    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods for RngCore here
    }

    let mut rng = TestRng;
    let p = -0.1;
    rng.random_bool(p);
}

#[test]
#[should_panic]
fn test_random_bool_with_zero_probability() {
    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods for RngCore here
    }

    let mut rng = TestRng;
    let p = 0.0;
    rng.random_bool(p);
}

#[test]
#[should_panic]
fn test_random_bool_with_one_probability() {
    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods for RngCore here
    }

    let mut rng = TestRng;
    let p = 1.0;
    rng.random_bool(p);
}

#[test]
#[should_panic]
fn test_random_bool_with_exceeding_probability() {
    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods for RngCore here
    }

    let mut rng = TestRng;
    let p = 1.1;
    rng.random_bool(p);
}


// Answer 0

#[test]
fn test_new_with_zero_threshold() {
    struct ValidRng;
    impl SeedableRng for ValidRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(ValidRng)
        }
    }
    struct ValidReseeder;
    impl TryRngCore for ValidReseeder {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(ValidReseeder)
        }
    }

    let threshold = 0;
    let reseeder = ValidReseeder;
    let _ = ReseedingCore::<ValidRng, ValidReseeder>::new(threshold, reseeder);
}

#[test]
fn test_new_with_max_threshold() {
    struct ValidRng;
    impl SeedableRng for ValidRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(ValidRng)
        }
    }
    struct ValidReseeder;
    impl TryRngCore for ValidReseeder {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(ValidReseeder)
        }
    }

    let threshold = i64::MAX as u64;
    let reseeder = ValidReseeder;
    let _ = ReseedingCore::<ValidRng, ValidReseeder>::new(threshold, reseeder);
}

#[test]
#[should_panic]
fn test_new_with_invalid_reseed() {
    struct ValidRng;
    impl SeedableRng for ValidRng {
        type Error = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Err(())
        }
    }
    struct InvalidReseeder;
    impl TryRngCore for InvalidReseeder {
        type Error = ();
    }

    let threshold = 0;
    let reseeder = InvalidReseeder;
    let _ = ReseedingCore::<ValidRng, InvalidReseeder>::new(threshold, reseeder);
}


// Answer 0

#[test]
fn test_new_inclusive_valid_range() {
    struct FiniteType;
    impl SampleUniform for FiniteType {
        type Sampler = FiniteSampler;
    }

    struct FiniteSampler;

    impl UniformSampler for FiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Ok(FiniteSampler)
        }
    }

    let _result = Uniform::<FiniteType>::new_inclusive(1.0, 5.0);
}

#[test]
fn test_new_inclusive_equal_low_high() {
    struct FiniteType;
    impl SampleUniform for FiniteType {
        type Sampler = FiniteSampler;
    }

    struct FiniteSampler;

    impl UniformSampler for FiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Ok(FiniteSampler)
        }
    }

    let _result = Uniform::<FiniteType>::new_inclusive(3.0, 3.0);
}

#[test]
fn test_new_inclusive_low_greater_high() {
    struct FiniteType;
    impl SampleUniform for FiniteType {
        type Sampler = FiniteSampler;
    }

    struct FiniteSampler;

    impl UniformSampler for FiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::EmptyRange)
        }
    }

    let _result = Uniform::<FiniteType>::new_inclusive(5.0, 2.0);
}

#[test]
fn test_new_inclusive_non_finite_low() {
    struct NonFiniteType;
    impl SampleUniform for NonFiniteType {
        type Sampler = NonFiniteSampler;
    }

    struct NonFiniteSampler;

    impl UniformSampler for NonFiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::NonFinite)
        }
    }

    let _result = Uniform::<NonFiniteType>::new_inclusive(std::f32::NAN, 5.0);
}

#[test]
fn test_new_inclusive_non_finite_high() {
    struct NonFiniteType;
    impl SampleUniform for NonFiniteType {
        type Sampler = NonFiniteSampler;
    }

    struct NonFiniteSampler;

    impl UniformSampler for NonFiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::NonFinite)
        }
    }

    let _result = Uniform::<NonFiniteType>::new_inclusive(1.0, std::f32::NAN);
}

#[test]
fn test_new_inclusive_infinite_low() {
    struct InfiniteType;
    impl SampleUniform for InfiniteType {
        type Sampler = InfiniteSampler;
    }

    struct InfiniteSampler;

    impl UniformSampler for InfiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::NonFinite)
        }
    }

    let _result = Uniform::<InfiniteType>::new_inclusive(std::f32::NEG_INFINITY, 5.0);
}

#[test]
fn test_new_inclusive_infinite_high() {
    struct InfiniteType;
    impl SampleUniform for InfiniteType {
        type Sampler = InfiniteSampler;
    }

    struct InfiniteSampler;

    impl UniformSampler for InfiniteSampler {
        fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Result<Self, Error> {
            Err(Error::NonFinite)
        }
    }

    let _result = Uniform::<InfiniteType>::new_inclusive(1.0, std::f32::INFINITY);
}


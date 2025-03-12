// Answer 0

#[test]
fn test_uniform_new_valid_range() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStruct;
    }

    let result = Uniform::<SampleStruct>::new(1, 10);
    let _ = result.unwrap(); // Unwrap to ensure it is Ok(Uniform<X>)
}

#[test]
fn test_uniform_new_valid_float_range() {
    struct SampleFloat;

    impl SampleUniform for SampleFloat {
        type Sampler = SampleFloat;
    }

    let result = Uniform::<SampleFloat>::new(0.1f32, 0.5f32);
    let _ = result.unwrap(); // Unwrap to ensure it is Ok(Uniform<X>)
}

#[test]
#[should_panic]
fn test_uniform_new_invalid_equal_values() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStruct;
    }

    let _ = Uniform::<SampleStruct>::new(5, 5); // Should panic
}

#[test]
#[should_panic]
fn test_uniform_new_invalid_low_greater_than_high() {
    struct SampleStruct;

    impl SampleUniform for SampleStruct {
        type Sampler = SampleStruct;
    }

    let _ = Uniform::<SampleStruct>::new(10, 5); // Should panic
}

#[test]
#[should_panic]
fn test_uniform_new_invalid_nan() {
    struct SampleFloat;

    impl SampleUniform for SampleFloat {
        type Sampler = SampleFloat;
    }

    let _ = Uniform::<SampleFloat>::new(f32::NAN, 10.0); // Should panic
}

#[test]
#[should_panic]
fn test_uniform_new_invalid_infinity() {
    struct SampleFloat;

    impl SampleUniform for SampleFloat {
        type Sampler = SampleFloat;
    }

    let _ = Uniform::<SampleFloat>::new(0.0, f32::INFINITY); // Should panic
}


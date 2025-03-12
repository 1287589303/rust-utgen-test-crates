// Answer 0

#[test]
fn test_sample_single_inclusive_integers() {
    struct SimpleRng;
    
    impl RngCore for SimpleRng {
        // Implement required methods...
    }
    
    let mut rng = SimpleRng;
    
    let result1 = UniformSampler::sample_single_inclusive(1, 10, &mut rng);
    let result2 = UniformSampler::sample_single_inclusive(0, 0, &mut rng);
}

#[test]
fn test_sample_single_inclusive_floats() {
    struct SimpleRng;
    
    impl RngCore for SimpleRng {
        // Implement required methods...
    }
    
    let mut rng = SimpleRng;
    
    let result1 = UniformSampler::sample_single_inclusive(1.0, 10.0, &mut rng);
    let result2 = UniformSampler::sample_single_inclusive(5.5, 5.5, &mut rng);
}

#[test]
#[should_panic]
fn test_sample_single_inclusive_empty_range() {
    struct SimpleRng;
    
    impl RngCore for SimpleRng {
        // Implement required methods...
    }
    
    let mut rng = SimpleRng;
    
    let _ = UniformSampler::sample_single_inclusive(10, 1, &mut rng);
}

#[test]
fn test_sample_single_inclusive_char() {
    struct SimpleRng;
    
    impl RngCore for SimpleRng {
        // Implement required methods...
    }
    
    let mut rng = SimpleRng;
    
    let result1 = UniformSampler::sample_single_inclusive('a', 'z', &mut rng);
    let result2 = UniformSampler::sample_single_inclusive('g', 'g', &mut rng);
}

#[test]
#[should_panic]
fn test_sample_single_inclusive_non_finite() {
    struct SimpleRng;
    
    impl RngCore for SimpleRng {
        // Implement required methods...
    }
    
    let mut rng = SimpleRng;
    
    let _ = UniformSampler::sample_single_inclusive(f32::INFINITY, 1.0, &mut rng);
}


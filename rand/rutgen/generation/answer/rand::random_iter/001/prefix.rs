// Answer 0

#[test]
fn test_random_iter_i32() {
    let random_values: Vec<i32> = rand::random_iter().take(1).collect();
}

#[test]
fn test_random_iter_f64() {
    let random_values: Vec<f64> = rand::random_iter().take(1).collect();
}

#[test]
fn test_random_iter_i32_multiple() {
    let random_values: Vec<i32> = rand::random_iter().take(5).collect();
}

#[test]
fn test_random_iter_f64_multiple() {
    let random_values: Vec<f64> = rand::random_iter().take(5).collect();
}

#[test]
fn test_random_iter_i32_maximum() {
    let random_values: Vec<i32> = rand::random_iter().take(10).collect();
}

#[test]
fn test_random_iter_f64_maximum() {
    let random_values: Vec<f64> = rand::random_iter().take(10).collect();
}

#[derive(Debug, Clone, Copy)]
struct CustomType(f64);

impl rand::distr::Distribution<CustomType> for StandardUniform {
    fn sample<R: RngCore + CryptoRng>(&self, rng: &mut R) -> CustomType {
        CustomType(rng.next_u64() as f64)
    }
}

#[test]
fn test_random_iter_custom_type() {
    let random_values: Vec<CustomType> = rand::random_iter().take(1).collect();
}

#[test]
fn test_random_iter_custom_type_multiple() {
    let random_values: Vec<CustomType> = rand::random_iter().take(5).collect();
}

#[test]
fn test_random_iter_custom_type_maximum() {
    let random_values: Vec<CustomType> = rand::random_iter().take(10).collect();
}


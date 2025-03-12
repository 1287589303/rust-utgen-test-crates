// Answer 0

#[test]
fn test_next_u32_initialized_seeded() {
    let rng = StdRng(Rng::from_seed([0u8; 32]));
    let value: u32 = rng.next_u32();
}

#[test]
fn test_next_u32_initialized_unseeded() {
    let rng = StdRng(Rng::from_entropy());
    let value: u32 = rng.next_u32();
}

#[test]
fn test_next_u32_max_value() {
    let rng = StdRng(Rng::from_seed([255u8; 32]));
    let value: u32 = rng.next_u32();
}

#[test]
fn test_next_u32_min_value() {
    let rng = StdRng(Rng::from_seed([0u8; 32]));
    let value: u32 = rng.next_u32();
}

#[test]
fn test_next_u32_repeated_calls() {
    let rng = StdRng(Rng::from_seed([1u8; 32]));
    let value1: u32 = rng.next_u32();
    let value2: u32 = rng.next_u32();
    let value3: u32 = rng.next_u32();
}

#[test]
fn test_next_u32_output_type() {
    let rng = StdRng(Rng::from_seed([2u8; 32]));
    let value: u32 = rng.next_u32();
}


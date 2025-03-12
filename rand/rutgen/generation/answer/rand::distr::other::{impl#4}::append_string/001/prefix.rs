// Answer 0

#[test]
fn test_append_string_zero_length() {
    use crate::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(0); // Initialize a default RNG
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 0);
}

#[test]
fn test_append_string_small_length() {
    use crate::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(1); // Initialize a default RNG
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 1);
}

#[test]
fn test_append_string_medium_length() {
    use crate::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(2); // Initialize a default RNG
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 50);
}

#[test]
fn test_append_string_max_length() {
    use crate::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(3); // Initialize a default RNG
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 100);
}

#[test]
fn test_append_string_exceeding_length() {
    use crate::rngs::StdRng;

    let mut rng = StdRng::seed_from_u64(4); // Initialize a default RNG
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 101); // Testing exceeding 100
}


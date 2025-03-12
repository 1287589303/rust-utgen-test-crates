// Answer 0

#[test]
fn test_f64_from_parts_exceeds_range_positive() {
    let mut deserializer = Deserializer {
        read: /* appropriate read struct */,
        scratch: Vec::new(),
        remaining_depth: 0,
        // additional fields if necessary for the context
    };
    let positive = true;
    let significand = 1u64;
    let exponent = 0i32;
    
    let result = deserializer.f64_from_parts(positive, significand, exponent);
}

#[test]
fn test_f64_from_parts_exceeds_range_negative() {
    let mut deserializer = Deserializer {
        read: /* appropriate read struct */,
        scratch: Vec::new(),
        remaining_depth: 0,
        // additional fields if necessary for the context
    };
    let positive = false;
    let significand = 1u64;
    let exponent = 0i32;
    
    let result = deserializer.f64_from_parts(positive, significand, exponent);
}


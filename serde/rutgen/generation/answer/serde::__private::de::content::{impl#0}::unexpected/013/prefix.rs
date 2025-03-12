// Answer 0

#[test]
fn test_unexpected_f32_values() {
    let values: [f32; 5] = [
        f32::MAX,       // Maximum f32 value
        f32::MIN,       // Minimum f32 value
        0.0,            // Zero value
        f32::NAN,       // Not a Number
        f32::INFINITY,  // Positive Infinity
    ];

    for &value in &values {
        let content = Content::F32(value);
        let _ = content.unexpected(); // Call the method
    }
}


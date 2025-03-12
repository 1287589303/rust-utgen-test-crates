// Answer 0

#[test]
fn test_format_finite_float() {
    struct FloatWrapper(f32);
    
    impl Float for FloatWrapper {
        fn is_nonfinite(&self) -> bool {
            false
        }
        
        fn format_nonfinite(&self) -> &str {
            unreachable!()
        }
    }
    
    let mut buffer = String::new();
    
    let float_value = FloatWrapper(3.14);
    let result = buffer.format(float_value);
    
    assert_eq!(result, buffer.format_finite(float_value));
}

#[test]
fn test_format_finite_double() {
    struct FloatWrapper(f64);
    
    impl Float for FloatWrapper {
        fn is_nonfinite(&self) -> bool {
            false
        }
        
        fn format_nonfinite(&self) -> &str {
            unreachable!()
        }
    }
    
    let mut buffer = String::new();
    
    let float_value = FloatWrapper(2.718281828);
    let result = buffer.format(float_value);
    
    assert_eq!(result, buffer.format_finite(float_value));
}


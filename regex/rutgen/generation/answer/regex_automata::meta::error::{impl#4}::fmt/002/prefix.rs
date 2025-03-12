// Answer 0

#[test]
fn test_fmt_quadratic_error() {
    struct LocalRetryQuadraticError(());
    
    let quadratic_error = LocalRetryQuadraticError(());
    let error = RetryError::Quadratic(quadratic_error);
    
    let mut output = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);
    
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_fail_error() {
    struct LocalRetryFailError {
        offset: usize,
    }
    
    impl core::fmt::Display for LocalRetryFailError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "regex engine failed at offset {:?}", self.offset)
        }
    }
    
    let fail_error = LocalRetryFailError { offset: 42 };
    let error = RetryError::Fail(fail_error);
    
    let mut output = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);
    
    let _ = error.fmt(&mut formatter);
}


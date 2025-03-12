fn from(err: RetryQuadraticError) -> RetryError {
        RetryError::Quadratic(err)
    }
// Answer 0

#[test]
fn test_check_size_limit_exceeding_high_memory_usage() {
    let mut builder = Builder::new();
    builder.set_size_limit(Some(1024)).unwrap(); // Set size limit to 1024
    builder.memory_states = 2048; // Simulate high memory usage

    let _ = builder.check_size_limit(); // Should return Err(BuildError::exceeded_size_limit(1024))
}

#[test]
fn test_check_size_limit_exceeding_memory_usage_with_limit() {
    let mut builder = Builder::new();
    builder.set_size_limit(Some(512)).unwrap(); // Set size limit to 512
    builder.memory_states = 1024; // Simulate memory usage exceeding limit

    let _ = builder.check_size_limit(); // Should return Err(BuildError::exceeded_size_limit(512))
}

#[test]
fn test_check_size_limit_with_non_zero_size_limit() {
    let mut builder = Builder::new();
    builder.set_size_limit(Some(256)).unwrap(); // Set a small size limit
    builder.memory_states = 512; // Set memory usage to exceed the limit

    let _ = builder.check_size_limit(); // Should return Err(BuildError::exceeded_size_limit(256))
}


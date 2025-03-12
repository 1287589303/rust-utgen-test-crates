// Answer 0

#[test]
fn test_memory_usage_zero() {
    let memchr = Memchr3(1, 2, 3);
    let usage = memchr.memory_usage();
}

#[test]
fn test_memory_usage_zero_with_different_values() {
    let memchr = Memchr3(0, 0, 0);
    let usage = memchr.memory_usage();
}

#[test]
fn test_memory_usage_zero_with_large_values() {
    let memchr = Memchr3(u8::MAX, u8::MAX, u8::MAX);
    let usage = memchr.memory_usage();
}


// Answer 0

#[test]
fn test_memory_usage_empty() {
    let byteset = ByteSet([false; 256]);
    let _ = byteset.memory_usage();
}

#[test]
fn test_memory_usage_full() {
    let byteset = ByteSet([true; 256]);
    let _ = byteset.memory_usage();
}

#[test]
fn test_memory_usage_mixed() {
    let byteset = ByteSet([
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false,
    ]);
    let _ = byteset.memory_usage();
}

#[test]
fn test_memory_usage_alternate() {
    let byteset = ByteSet([
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false,
    ]);
    let _ = byteset.memory_usage();
}


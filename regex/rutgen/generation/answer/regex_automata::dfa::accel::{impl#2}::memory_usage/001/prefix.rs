// Answer 0

#[test]
fn test_memory_usage_zero_accels() {
    let accels = Accels { accels: vec![0u32; 0] }; 
    let usage = accels.memory_usage();
}

#[test]
fn test_memory_usage_one_accel() {
    let accels = Accels { accels: vec![1u32] }; 
    let usage = accels.memory_usage();
}

#[test]
fn test_memory_usage_four_accels() {
    let accels = Accels { accels: vec![1u32, 2, 3, 4] }; 
    let usage = accels.memory_usage();
}

#[test]
fn test_memory_usage_eight_accels() {
    let accels = Accels { accels: vec![1u32, 2, 3, 4, 5, 6, 7, 8] }; 
    let usage = accels.memory_usage();
}

#[test]
fn test_memory_usage_full_capacity() {
    let accels = Accels { accels: vec![0u32; 8] }; 
    let usage = accels.memory_usage();
}


// Answer 0

#[test]
fn test_put_bytes_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0); // Ensures there's enough capacity for the operation
    let val: u8 = 42;
    let cnt: usize = 10; // Set cnt to match the spare capacity
    bytes_mut.put_bytes(val, cnt);
}

#[test]
fn test_put_bytes_single_byte() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    bytes_mut.resize(1, 0); // Resize to ensure enough space
    let val: u8 = 255;
    let cnt: usize = 1; // Single byte write
    bytes_mut.put_bytes(val, cnt);
}

#[test]
fn test_put_bytes_max_spare_capacity() {
    let max_capacity = 255; // Assuming maximum is defined for context
    let mut bytes_mut = BytesMut::with_capacity(max_capacity);
    bytes_mut.resize(max_capacity, 0); // Ensures there's enough capacity
    let val: u8 = 100;
    let cnt: usize = max_capacity; // Use maximum spare capacity
    bytes_mut.put_bytes(val, cnt);
}


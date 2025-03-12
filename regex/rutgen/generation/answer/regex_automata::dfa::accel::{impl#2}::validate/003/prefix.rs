// Answer 0

#[test]
fn test_validate_with_valid_accelerators() {
    let accels = Accels {
        accels: vec![0u32, 1u32, 2u32, 3u32],
    };
    let result = accels.validate();
}

#[test]
fn test_validate_with_single_chunk_smaller_than_accel_cap() {
    let bytes: [u8; 5] = [4, 0, 1, 2, 3]; // 4 indicates length, followed by 4 bytes (should create a valid chunk)
    let accels = Accels {
        accels: bytes.as_ref(),
    };
    let result = accels.validate();
}

#[test]
fn test_validate_with_last_chunk_smaller_than_accel_len() {
    let bytes: [u8; 9] = [5, 0, 1, 2, 3, 4, 5, 6, 7]; // 5 indicates length, with one last byte less than ACCEL_LEN
    let accels = Accels {
        accels: bytes.as_ref(),
    };
    let result = accels.validate();
}

#[test]
fn test_validate_with_multiple_chunks_with_valid_sizes() {
    let bytes: [u8; 16] = [3, 0, 1, 2, 3, 4, 5, 6, 7, 8]; // 3 accelerators, the rest creates chunks without exceeding limits
    let accels = Accels {
        accels: bytes.as_ref(),
    };
    let result = accels.validate();
}


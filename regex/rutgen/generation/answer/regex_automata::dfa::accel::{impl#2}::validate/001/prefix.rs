// Answer 0

#[test]
fn test_validate_accels_invalid_length() {
    let invalid_length_bytes: Vec<u8> = vec![0; ACCEL_TY_SIZE + ACCEL_CAP - 1]; // Invalid length, less than ACCEL_CAP
    let accels = Accels { accels: &invalid_length_bytes };
    let _ = accels.validate();
}

#[test]
fn test_validate_accels_with_invalid_chunk() {
    let invalid_chunk_bytes: Vec<u8> = vec![0; ACCEL_TY_SIZE + 3 * ACCEL_CAP]; // Create length to include 3 chunks
    // Modify the last chunk to be invalid (not enough bytes for Accel)
    invalid_chunk_bytes[ACCEL_TY_SIZE + 2 * ACCEL_CAP..ACCEL_TY_SIZE + 3 * ACCEL_CAP].fill(0); // Assuming this doesn't form a valid structure
    let accels = Accels { accels: &invalid_chunk_bytes };
    let _ = accels.validate();
}

#[test]
fn test_validate_accels_multiple_invalid_chunks() {
    let mut invalid_chunks_bytes: Vec<u8> = vec![0; ACCEL_TY_SIZE + 2 * ACCEL_CAP]; // Create 2 valid chunks
    // Create a larger invalid chunk
    invalid_chunks_bytes.extend_from_slice(&[0; ACCEL_CAP - 1]); // Invalid last chunk

    let accels = Accels { accels: &invalid_chunks_bytes };
    let _ = accels.validate();
}


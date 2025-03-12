// Answer 0

#[derive(Copy, Clone)]
struct TestType(u32);

impl Observable for TestType {
    type Bytes = [u8; 4];
    fn to_le_bytes(self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
}

#[test]
fn test_fill_via_chunks_multiple() {
    let src: &[TestType] = &[TestType(1), TestType(2)];
    let mut dest: [u8; 8] = [0; 8];
    let result = fill_via_chunks(src, &mut dest);
    let expected_chunks = 2; // Two elements in src
    let expected_bytes = 8;   // 2 * size of TestType (4 bytes)
    let _ = (result.0, expected_chunks);
    let _ = (result.1, expected_bytes);
}

#[test]
fn test_fill_via_chunks_with_remainder_zero() {
    let src: &[TestType] = &[TestType(3)];
    let mut dest: [u8; 4] = [0; 4]; // Exact size of one TestType
    let result = fill_via_chunks(src, &mut dest);
    let expected_chunks = 1; // One element in src
    let expected_bytes = 4;  // 1 * size of TestType (4 bytes)
    let _ = (result.0, expected_chunks);
    let _ = (result.1, expected_bytes);
}


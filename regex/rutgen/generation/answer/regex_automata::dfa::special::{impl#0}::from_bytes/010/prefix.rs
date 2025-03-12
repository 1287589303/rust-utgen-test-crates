// Answer 0

#[test]
fn test_from_bytes_valid() {
    let bytes = vec![0; 8 * std::mem::size_of::<StateID>()];
    let _ = Special::from_bytes(&bytes);
}

#[test]
fn test_from_bytes_min_match_equals_max_match() {
    let mut bytes = vec![0; 8 * std::mem::size_of::<StateID>()];
    bytes[0..std::mem::size_of::<StateID>()].copy_from_slice(&0u32.to_ne_bytes()); // max
    bytes[std::mem::size_of::<StateID>()..2 * std::mem::size_of::<StateID>()].copy_from_slice(&1u32.to_ne_bytes()); // quit_id
    bytes[2 * std::mem::size_of::<StateID>()..3 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // min_match
    bytes[3 * std::mem::size_of::<StateID>()..4 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // max_match
    bytes[4 * std::mem::size_of::<StateID>()..5 * std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // min_accel
    bytes[5 * std::mem::size_of::<StateID>()..6 * std::mem::size_of::<StateID>()].copy_from_slice(&4u32.to_ne_bytes()); // max_accel
    bytes[6 * std::mem::size_of::<StateID>()..7 * std::mem::size_of::<StateID>()].copy_from_slice(&5u32.to_ne_bytes()); // min_start
    bytes[7 * std::mem::size_of::<StateID>()..8 * std::mem::size_of::<StateID>()].copy_from_slice(&6u32.to_ne_bytes()); // max_start
    let _ = Special::from_bytes(&bytes);
}

#[test]
fn test_from_bytes_max_equals_quit_id() {
    let mut bytes = vec![0; 8 * std::mem::size_of::<StateID>()];
    bytes[0..std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // max
    bytes[std::mem::size_of::<StateID>()..2 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // quit_id
    bytes[2 * std::mem::size_of::<StateID>()..3 * std::mem::size_of::<StateID>()].copy_from_slice(&1u32.to_ne_bytes()); // min_match
    bytes[3 * std::mem::size_of::<StateID>()..4 * std::mem::size_of::<StateID>()].copy_from_slice(&4u32.to_ne_bytes()); // max_match
    bytes[4 * std::mem::size_of::<StateID>()..5 * std::mem::size_of::<StateID>()].copy_from_slice(&5u32.to_ne_bytes()); // min_accel
    bytes[5 * std::mem::size_of::<StateID>()..6 * std::mem::size_of::<StateID>()].copy_from_slice(&6u32.to_ne_bytes()); // max_accel
    bytes[6 * std::mem::size_of::<StateID>()..7 * std::mem::size_of::<StateID>()].copy_from_slice(&7u32.to_ne_bytes()); // min_start
    bytes[7 * std::mem::size_of::<StateID>()..8 * std::mem::size_of::<StateID>()].copy_from_slice(&8u32.to_ne_bytes()); // max_start
    let _ = Special::from_bytes(&bytes);
}

#[test]
fn test_from_bytes_invalid_min_match_greater_than_max_match() {
    let mut bytes = vec![0; 8 * std::mem::size_of::<StateID>()];
    bytes[0..std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // max
    bytes[std::mem::size_of::<StateID>()..2 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // quit_id
    bytes[2 * std::mem::size_of::<StateID>()..3 * std::mem::size_of::<StateID>()].copy_from_slice(&5u32.to_ne_bytes()); // min_match
    bytes[3 * std::mem::size_of::<StateID>()..4 * std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // max_match
    bytes[4 * std::mem::size_of::<StateID>()..5 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // min_accel
    bytes[5 * std::mem::size_of::<StateID>()..6 * std::mem::size_of::<StateID>()].copy_from_slice(&4u32.to_ne_bytes()); // max_accel
    bytes[6 * std::mem::size_of::<StateID>()..7 * std::mem::size_of::<StateID>()].copy_from_slice(&5u32.to_ne_bytes()); // min_start
    bytes[7 * std::mem::size_of::<StateID>()..8 * std::mem::size_of::<StateID>()].copy_from_slice(&6u32.to_ne_bytes()); // max_start
    let result = Special::from_bytes(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_quit_id_greater_than_min_match() {
    let mut bytes = vec![0; 8 * std::mem::size_of::<StateID>()];
    bytes[0..std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // max
    bytes[std::mem::size_of::<StateID>()..2 * std::mem::size_of::<StateID>()].copy_from_slice(&4u32.to_ne_bytes()); // quit_id
    bytes[2 * std::mem::size_of::<StateID>()..3 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // min_match
    bytes[3 * std::mem::size_of::<StateID>()..4 * std::mem::size_of::<StateID>()].copy_from_slice(&5u32.to_ne_bytes()); // max_match
    bytes[4 * std::mem::size_of::<StateID>()..5 * std::mem::size_of::<StateID>()].copy_from_slice(&1u32.to_ne_bytes()); // min_accel
    bytes[5 * std::mem::size_of::<StateID>()..6 * std::mem::size_of::<StateID>()].copy_from_slice(&2u32.to_ne_bytes()); // max_accel
    bytes[6 * std::mem::size_of::<StateID>()..7 * std::mem::size_of::<StateID>()].copy_from_slice(&3u32.to_ne_bytes()); // min_start
    bytes[7 * std::mem::size_of::<StateID>()..8 * std::mem::size_of::<StateID>()].copy_from_slice(&4u32.to_ne_bytes()); // max_start
    let result = Special::from_bytes(&bytes);
    assert!(result.is_err());
}


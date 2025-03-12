// Answer 0

#[test]
fn test_next_with_non_empty_slots() {
    let mut slots = Slots(0b0111); // Binary representation with trailing zeros
    let mut iter = SlotsIter { slots };
    let result = iter.next();
    let expected_slot = 0; // First trailing zero index
    assert_eq!(result, Some(expected_slot));
}

#[test]
fn test_next_with_specific_slot() {
    let mut slots = Slots(0b1010); // Binary representation with trailing zeros
    let mut iter = SlotsIter { slots };
    let result = iter.next();
    let expected_slot = 1; // Second trailing zero index
    assert_eq!(result, Some(expected_slot));
}

#[test]
fn test_next_with_multiple_trailing_zeros() {
    let mut slots = Slots(0b00011100); // Binary representation with trailing zeros
    let mut iter = SlotsIter { slots };
    let result = iter.next();
    let expected_slot = 2; // First available slot with three trailing zeros
    assert_eq!(result, Some(expected_slot));
}

#[test]
fn test_next_with_full_slots() {
    let mut slots = Slots(0b11111111); // No trailing zeros
    let mut iter = SlotsIter { slots };
    let result = iter.next();
    assert_eq!(result, Some(0));
    
    let next_result = iter.next(); // Should return next slot
    assert_eq!(next_result, Some(1));
}

#[test]
fn test_next_edge_case() {
    let mut slots = Slots(0b00000001); // Only one slot
    let mut iter = SlotsIter { slots };
    let result = iter.next();
    let expected_slot = 0; // Only one available slot
    assert_eq!(result, Some(expected_slot));
}


// Answer 0

#[test]
fn test_state_byte_range() {
    let state = State::ByteRange {
        trans: Transition {
            byte: 128, // Choose a byte value within the valid range
            next: StateID(SmallIndex::new_unchecked(1)), // A valid next state ID
        },
    };

    let mut formatter = fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_state_byte_range_with_zero() {
    let state = State::ByteRange {
        trans: Transition {
            byte: 0, // Edge case: the lowest byte value
            next: StateID(SmallIndex::new_unchecked(0)), // Next state ID set to zero
        },
    };

    let mut formatter = fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_state_byte_range_with_max() {
    let state = State::ByteRange {
        trans: Transition {
            byte: 255, // Edge case: the highest byte value
            next: StateID(SmallIndex::new_unchecked(UINT_MAX as usize)), // Next state ID set to max
        },
    };

    let mut formatter = fmt::Formatter::new();
    let _ = state.fmt(&mut formatter);
}


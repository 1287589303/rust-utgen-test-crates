// Answer 0

#[test]
fn test_next_back_valid_pattern_id_1() {
    #[cfg(feature = "alloc")]
    {
        let pattern_set = [true, false, false, true];
        let iter = PatternSetIter {
            it: pattern_set.iter().enumerate(),
        };
        let mut iter = iter.clone();
        let _result = iter.next_back(); // should return Some(PatternID::new_unchecked(3))
    }
}

#[test]
fn test_next_back_valid_pattern_id_2() {
    #[cfg(feature = "alloc")]
    {
        let pattern_set = [false, true, false, false];
        let iter = PatternSetIter {
            it: pattern_set.iter().enumerate(),
        };
        let mut iter = iter.clone();
        let _result = iter.next_back(); // should return Some(PatternID::new_unchecked(1))
    }
}

#[test]
fn test_next_back_valid_pattern_id_3() {
    #[cfg(feature = "alloc")]
    {
        let pattern_set = [true, true, false];
        let iter = PatternSetIter {
            it: pattern_set.iter().enumerate(),
        };
        let mut iter = iter.clone();
        let _result = iter.next_back(); // should return Some(PatternID::new_unchecked(1))
    }
}

#[test]
fn test_next_back_valid_pattern_id_boundary() {
    #[cfg(feature = "alloc")]
    {
        let pattern_set = [false, false, true];
        let iter = PatternSetIter {
            it: pattern_set.iter().enumerate(),
        };
        let mut iter = iter.clone();
        let _result = iter.next_back(); // should return Some(PatternID::new_unchecked(2))
    }
}


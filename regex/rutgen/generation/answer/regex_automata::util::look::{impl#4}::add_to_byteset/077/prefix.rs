// Answer 0

#[test]
fn test_add_to_byteset_word_ascii_negate_b1_exceeds_range() {
    let mut byte_class_set = {
        #[cfg(feature = "alloc")]
        {
            crate::util::alphabet::ByteClassSet::empty()
        }
    };
    let look = Look::WordAsciiNegate;
    let look_matcher = LookMatcher::new();

    look_matcher.add_to_byteset(look, &mut byte_class_set);
}

#[test]
fn test_add_to_byteset_word_ascii_negate_b1_at_upper_boundary() {
    let mut byte_class_set = {
        #[cfg(feature = "alloc")]
        {
            crate::util::alphabet::ByteClassSet::empty()
        }
    };
    let look = Look::WordAsciiNegate;
    let look_matcher = LookMatcher::new();

    look_matcher.add_to_byteset(look, &mut byte_class_set);
    // This call is expected to run without panic, covering the `b1 <= 255` condition
}


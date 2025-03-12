fn from(_: crate::punycode::PunycodeEncodeError) -> Self {
        unreachable!(
            "Punycode overflows should not be possible due to PUNYCODE_ENCODE_MAX_INPUT_LENGTH"
        );
    }
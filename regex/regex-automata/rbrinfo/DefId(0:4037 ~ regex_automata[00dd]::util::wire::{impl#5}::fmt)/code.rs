fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::DeserializeErrorKind::*;

        match self.0 {
            Generic { msg } => write!(f, "{}", msg),
            BufferTooSmall { what } => {
                write!(f, "buffer is too small to read {}", what)
            }
            InvalidUsize { what } => {
                write!(f, "{} is too big to fit in a usize", what)
            }
            VersionMismatch { expected, found } => write!(
                f,
                "unsupported version: \
                 expected version {} but found version {}",
                expected, found,
            ),
            EndianMismatch { expected, found } => write!(
                f,
                "endianness mismatch: expected 0x{:X} but got 0x{:X}. \
                 (Are you trying to load an object serialized with a \
                 different endianness?)",
                expected, found,
            ),
            AlignmentMismatch { alignment, address } => write!(
                f,
                "alignment mismatch: slice starts at address \
                 0x{:X}, which is not aligned to a {} byte boundary",
                address, alignment,
            ),
            LabelMismatch { expected } => write!(
                f,
                "label mismatch: start of serialized object should \
                 contain a NUL terminated {:?} label, but a different \
                 label was found",
                expected,
            ),
            ArithmeticOverflow { what } => {
                write!(f, "arithmetic overflow for {}", what)
            }
            PatternID { ref err, what } => {
                write!(f, "failed to read pattern ID for {}: {}", what, err)
            }
            StateID { ref err, what } => {
                write!(f, "failed to read state ID for {}: {}", what, err)
            }
        }
    }
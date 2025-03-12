#[cfg(target_endian = "little")]
pub(crate) type NE = LE;
#[cfg(target_endian = "big")]
pub(crate) type NE = BE;
use core::{cmp, mem::size_of};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use crate::util::{
    int::Pointer, primitives::{PatternID, PatternIDError, StateID, StateIDError},
};
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
#[derive(Debug)]
enum DeserializeErrorKind {
    Generic { msg: &'static str },
    BufferTooSmall { what: &'static str },
    InvalidUsize { what: &'static str },
    VersionMismatch { expected: u32, found: u32 },
    EndianMismatch { expected: u32, found: u32 },
    AlignmentMismatch { alignment: usize, address: usize },
    LabelMismatch { expected: &'static str },
    ArithmeticOverflow { what: &'static str },
    PatternID { err: PatternIDError, what: &'static str },
    StateID { err: StateIDError, what: &'static str },
}
impl core::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::DeserializeErrorKind::*;
        match self.0 {
            Generic { msg } => write!(f, "{}", msg),
            BufferTooSmall { what } => write!(f, "buffer is too small to read {}", what),
            InvalidUsize { what } => write!(f, "{} is too big to fit in a usize", what),
            VersionMismatch { expected, found } => {
                write!(
                    f,
                    "unsupported version: \
                 expected version {} but found version {}",
                    expected, found,
                )
            }
            EndianMismatch { expected, found } => {
                write!(
                    f,
                    "endianness mismatch: expected 0x{:X} but got 0x{:X}. \
                 (Are you trying to load an object serialized with a \
                 different endianness?)",
                    expected, found,
                )
            }
            AlignmentMismatch { alignment, address } => {
                write!(
                    f,
                    "alignment mismatch: slice starts at address \
                 0x{:X}, which is not aligned to a {} byte boundary",
                    address, alignment,
                )
            }
            LabelMismatch { expected } => {
                write!(
                    f,
                    "label mismatch: start of serialized object should \
                 contain a NUL terminated {:?} label, but a different \
                 label was found",
                    expected,
                )
            }
            ArithmeticOverflow { what } => write!(f, "arithmetic overflow for {}", what),
            PatternID { ref err, what } => {
                write!(f, "failed to read pattern ID for {}: {}", what, err)
            }
            StateID { ref err, what } => {
                write!(f, "failed to read state ID for {}: {}", what, err)
            }
        }
    }
}

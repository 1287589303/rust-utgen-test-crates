#[cfg(feature = "dfa-build")]
use core::iter;
use core::{fmt, mem::size_of};
#[cfg(feature = "dfa-build")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "dfa-build")]
use crate::dfa::dense::{self, BuildError};
use crate::{
    dfa::{
        automaton::{fmt_state_indicator, Automaton, StartError},
        dense::Flags, special::Special, StartKind, DEAD,
    },
    util::{
        alphabet::{ByteClasses, ByteSet},
        escape::DebugByte, int::{Pointer, Usize, U16, U32},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-sparse";
const VERSION: u32 = 2;
#[derive(Debug)]
struct Seen {
    #[cfg(feature = "alloc")]
    set: alloc::collections::BTreeSet<StateID>,
    #[cfg(not(feature = "alloc"))]
    set: core::marker::PhantomData<StateID>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[cfg(not(feature = "alloc"))]
impl Seen {
    fn new() -> Seen {
        Seen {
            set: core::marker::PhantomData,
        }
    }
    fn insert(&mut self, _id: StateID) {}
    fn contains(&self, _id: &StateID) -> bool {}
}

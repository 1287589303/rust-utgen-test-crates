use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
#[derive(Clone, Debug)]
struct NextInsert {
    /// The next state to begin inserting ranges. This state should be the
    /// state at which `ranges[0]` should be inserted.
    state_id: StateID,
    /// The ranges to insert. We used a fixed-size array here to avoid an
    /// allocation.
    ranges: [Utf8Range; 4],
    /// The number of valid ranges in the above array.
    len: u8,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl NextInsert {
    fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert {}
    fn push(
        trie: &mut RangeTrie,
        stack: &mut Vec<NextInsert>,
        ranges: &[Utf8Range],
    ) -> StateID {}
    fn state_id(&self) -> StateID {
        self.state_id
    }
    fn ranges(&self) -> &[Utf8Range] {}
}

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
#[derive(Clone)]
pub struct RangeTrie {
    /// The states in this trie. The first is always the shared final state.
    /// The second is always the root state. Otherwise, there is no
    /// particular order.
    states: Vec<State>,
    /// A free-list of states. When a range trie is cleared, all of its states
    /// are added to this list. Creating a new state reuses states from this
    /// list before allocating a new one.
    free: Vec<State>,
    /// A stack for traversing this trie to yield sequences of byte ranges in
    /// lexicographic order.
    iter_stack: RefCell<Vec<NextIter>>,
    /// A buffer that stores the current sequence during iteration.
    iter_ranges: RefCell<Vec<Utf8Range>>,
    /// A stack used for traversing the trie in order to (deeply) duplicate
    /// a state. States are recursively duplicated when ranges are split.
    dupe_stack: Vec<NextDupe>,
    /// A stack used for traversing the trie during insertion of a new
    /// sequence of byte ranges.
    insert_stack: Vec<NextInsert>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl NextInsert {
    fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert {
        let len = ranges.len();
        assert!(len > 0);
        assert!(len <= 4);
        let mut tmp = [Utf8Range { start: 0, end: 0 }; 4];
        tmp[..len].copy_from_slice(ranges);
        NextInsert {
            state_id,
            ranges: tmp,
            len: u8::try_from(len).unwrap(),
        }
    }
    fn push(
        trie: &mut RangeTrie,
        stack: &mut Vec<NextInsert>,
        ranges: &[Utf8Range],
    ) -> StateID {
        if ranges.is_empty() {
            FINAL
        } else {
            let next_id = trie.add_empty();
            stack.push(NextInsert::new(next_id, ranges));
            next_id
        }
    }
    fn state_id(&self) -> StateID {}
    fn ranges(&self) -> &[Utf8Range] {}
}
impl RangeTrie {
    pub fn new() -> RangeTrie {}
    pub fn clear(&mut self) {}
    pub fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {}
    pub fn insert(&mut self, ranges: &[Utf8Range]) {}
    pub fn add_empty(&mut self) -> StateID {
        let id = match StateID::try_from(self.states.len()) {
            Ok(id) => id,
            Err(_) => {
                panic!("too many sequences added to range trie");
            }
        };
        if let Some(mut state) = self.free.pop() {
            state.clear();
            self.states.push(state);
        } else {
            self.states.push(State { transitions: vec![] });
        }
        id
    }
    fn duplicate(&mut self, old_id: StateID) -> StateID {}
    fn add_transition(&mut self, from_id: StateID, range: Utf8Range, next_id: StateID) {}
    fn add_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {}
    fn set_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {}
    fn state(&self, id: StateID) -> &State {}
    fn state_mut(&mut self, id: StateID) -> &mut State {}
}

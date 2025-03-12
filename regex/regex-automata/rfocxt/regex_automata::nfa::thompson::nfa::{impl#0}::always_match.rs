use core::{fmt, mem};
use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};
#[cfg(feature = "syntax")]
use crate::nfa::thompson::{
    compiler::{Compiler, Config},
    error::BuildError,
};
use crate::{
    nfa::thompson::builder::Builder,
    util::{
        alphabet::{self, ByteClassSet, ByteClasses},
        captures::{GroupInfo, GroupInfoError},
        look::{Look, LookMatcher, LookSet},
        primitives::{IteratorIndexExt, PatternID, PatternIDIter, SmallIndex, StateID},
        sparse_set::SparseSet,
    },
};
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The ID of the pattern that we're currently building.
    ///
    /// Callers are required to set (and unset) this by calling
    /// {start,finish}_pattern. Otherwise, most methods will panic.
    pattern_id: Option<PatternID>,
    /// A sequence of intermediate NFA states. Once a state is added to this
    /// sequence, it is assigned a state ID equivalent to its index. Once a
    /// state is added, it is still expected to be mutated, e.g., to set its
    /// transition to a state that didn't exist at the time it was added.
    states: Vec<State>,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// A map from pattern ID to capture group index to name. (If no name
    /// exists, then a None entry is present. Thus, all capturing groups are
    /// present in this mapping.)
    ///
    /// The outer vec is indexed by pattern ID, while the inner vec is indexed
    /// by capture index offset for the corresponding pattern.
    ///
    /// The first capture group for each pattern is always unnamed and is thus
    /// always None.
    captures: Vec<Vec<Option<Arc<str>>>>,
    /// The combined memory used by each of the 'State's in 'states'. This
    /// only includes heap usage by each state, and not the size of the state
    /// itself. In other words, this tracks heap memory used that isn't
    /// captured via `size_of::<State>() * states.len()`.
    memory_states: usize,
    /// Whether this NFA only matches UTF-8 and whether regex engines using
    /// this NFA for searching should report empty matches that split a
    /// codepoint.
    utf8: bool,
    /// Whether this NFA should be matched in reverse or not.
    reverse: bool,
    /// The matcher to use for look-around assertions.
    look_matcher: LookMatcher,
    /// A size limit to respect when building an NFA. If the total heap memory
    /// of the intermediate NFA states exceeds (or would exceed) this amount,
    /// then an error is returned.
    size_limit: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Default)]
pub(super) struct Inner {
    /// The state sequence. This sequence is guaranteed to be indexable by all
    /// starting state IDs, and it is also guaranteed to contain at most one
    /// `Match` state for each pattern compiled into this NFA. (A pattern may
    /// not have a corresponding `Match` state if a `Match` state is impossible
    /// to reach.)
    states: Vec<State>,
    /// The anchored starting state of this NFA.
    start_anchored: StateID,
    /// The unanchored starting state of this NFA.
    start_unanchored: StateID,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// Info about the capturing groups in this NFA. This is responsible for
    /// mapping groups to slots, mapping groups to names and names to groups.
    group_info: GroupInfo,
    /// A representation of equivalence classes over the transitions in this
    /// NFA. Two bytes in the same equivalence class must not discriminate
    /// between a match or a non-match. This map can be used to shrink the
    /// total size of a DFA's transition table with a small match-time cost.
    ///
    /// Note that the NFA's transitions are *not* defined in terms of these
    /// equivalence classes. The NFA's transitions are defined on the original
    /// byte values. For the most part, this is because they wouldn't really
    /// help the NFA much since the NFA already uses a sparse representation
    /// to represent transitions. Byte classes are most effective in a dense
    /// representation.
    byte_class_set: ByteClassSet,
    /// This is generated from `byte_class_set`, and essentially represents the
    /// same thing but supports different access patterns. Namely, this permits
    /// looking up the equivalence class of a byte very cheaply.
    ///
    /// Ideally we would just store this, but because of annoying code
    /// structure reasons, we keep both this and `byte_class_set` around for
    /// now. I think I would prefer that `byte_class_set` were computed in the
    /// `Builder`, but right now, we compute it as states are added to the
    /// `NFA`.
    byte_classes: ByteClasses,
    /// Whether this NFA has a `Capture` state anywhere.
    has_capture: bool,
    /// When the empty string is in the language matched by this NFA.
    has_empty: bool,
    /// Whether UTF-8 mode is enabled for this NFA. Briefly, this means that
    /// all non-empty matches produced by this NFA correspond to spans of valid
    /// UTF-8, and any empty matches produced by this NFA that split a UTF-8
    /// encoded codepoint should be filtered out by the corresponding regex
    /// engine.
    utf8: bool,
    /// Whether this NFA is meant to be matched in reverse or not.
    reverse: bool,
    /// The matcher to be used for look-around assertions.
    look_matcher: LookMatcher,
    /// The union of all look-around assertions that occur anywhere within
    /// this NFA. If this set is empty, then it means there are precisely zero
    /// conditional epsilon transitions in the NFA.
    look_set_any: LookSet,
    /// The union of all look-around assertions that occur as a zero-length
    /// prefix for any of the patterns in this NFA.
    look_set_prefix_any: LookSet,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {
        let mut builder = Builder::new();
        let pid = builder.start_pattern().unwrap();
        assert_eq!(pid.as_usize(), 0);
        let start_id = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
        let end_id = builder.add_capture_end(StateID::ZERO, 0).unwrap();
        let match_id = builder.add_match().unwrap();
        builder.patch(start_id, end_id).unwrap();
        builder.patch(end_id, match_id).unwrap();
        let pid = builder.finish_pattern(start_id).unwrap();
        assert_eq!(pid.as_usize(), 0);
        builder.build(start_id, start_id).unwrap()
    }
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {}
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }
    pub fn clear(&mut self) {}
    pub fn build(
        &self,
        start_anchored: StateID,
        start_unanchored: StateID,
    ) -> Result<NFA, BuildError> {
        assert!(self.pattern_id.is_none(), "must call 'finish_pattern' first");
        debug!(
            "intermediate NFA compilation via builder is complete, \
             intermediate NFA size: {} states, {} bytes on heap",
            self.states.len(), self.memory_usage(),
        );
        let mut nfa = nfa::Inner::default();
        nfa.set_utf8(self.utf8);
        nfa.set_reverse(self.reverse);
        nfa.set_look_matcher(self.look_matcher.clone());
        let mut empties = vec![];
        let mut remap = vec![];
        remap.resize(self.states.len(), StateID::ZERO);
        nfa.set_starts(start_anchored, start_unanchored, &self.start_pattern);
        nfa.set_captures(&self.captures).map_err(BuildError::captures)?;
        for (sid, state) in self.states.iter().with_state_ids() {
            match *state {
                State::Empty { next } => {
                    empties.push((sid, next));
                }
                State::ByteRange { trans } => {
                    remap[sid] = nfa.add(nfa::State::ByteRange { trans });
                }
                State::Sparse { ref transitions } => {
                    remap[sid] = match transitions.len() {
                        0 => nfa.add(nfa::State::Fail),
                        1 => {
                            nfa.add(nfa::State::ByteRange {
                                trans: transitions[0],
                            })
                        }
                        _ => {
                            let transitions = transitions.to_vec().into_boxed_slice();
                            let sparse = SparseTransitions { transitions };
                            nfa.add(nfa::State::Sparse(sparse))
                        }
                    };
                }
                State::Look { look, next } => {
                    remap[sid] = nfa.add(nfa::State::Look { look, next });
                }
                State::CaptureStart { pattern_id, group_index, next } => {
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index");
                    let slot = SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa
                        .add(nfa::State::Capture {
                            next,
                            pattern_id,
                            group_index,
                            slot,
                        });
                }
                State::CaptureEnd { pattern_id, group_index, next } => {
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index")
                        .checked_add(1)
                        .unwrap();
                    let slot = SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa
                        .add(nfa::State::Capture {
                            next,
                            pattern_id,
                            group_index,
                            slot,
                        });
                }
                State::Union { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa
                            .add(nfa::State::BinaryUnion {
                                alt1: alternates[0],
                                alt2: alternates[1],
                            });
                    } else {
                        let alternates = alternates.to_vec().into_boxed_slice();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::UnionReverse { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa
                            .add(nfa::State::BinaryUnion {
                                alt1: alternates[1],
                                alt2: alternates[0],
                            });
                    } else {
                        let mut alternates = alternates.to_vec().into_boxed_slice();
                        alternates.reverse();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::Fail => {
                    remap[sid] = nfa.add(nfa::State::Fail);
                }
                State::Match { pattern_id } => {
                    remap[sid] = nfa.add(nfa::State::Match { pattern_id });
                }
            }
        }
        let mut remapped = vec![false; self.states.len()];
        for &(empty_id, empty_next) in empties.iter() {
            if remapped[empty_id] {
                continue;
            }
            let mut new_next = empty_next;
            while let Some(next) = self.states[new_next].goto() {
                new_next = next;
            }
            remap[empty_id] = remap[new_next];
            remapped[empty_id] = true;
            let mut next2 = empty_next;
            while let Some(next) = self.states[next2].goto() {
                remap[next2] = remap[new_next];
                remapped[next2] = true;
                next2 = next;
            }
        }
        nfa.remap(&remap);
        let final_nfa = nfa.into_nfa();
        debug!(
            "NFA compilation via builder complete, \
             final NFA size: {} states, {} bytes on heap, \
             has empty? {:?}, utf8? {:?}",
            final_nfa.states().len(), final_nfa.memory_usage(), final_nfa.has_empty(),
            final_nfa.is_utf8(),
        );
        Ok(final_nfa)
    }
    pub fn start_pattern(&mut self) -> Result<PatternID, BuildError> {
        assert!(self.pattern_id.is_none(), "must call 'finish_pattern' first");
        let proposed = self.start_pattern.len();
        let pid = PatternID::new(proposed)
            .map_err(|_| BuildError::too_many_patterns(proposed))?;
        self.pattern_id = Some(pid);
        self.start_pattern.push(StateID::ZERO);
        Ok(pid)
    }
    pub fn finish_pattern(
        &mut self,
        start_id: StateID,
    ) -> Result<PatternID, BuildError> {
        let pid = self.current_pattern_id();
        self.start_pattern[pid] = start_id;
        self.pattern_id = None;
        Ok(pid)
    }
    pub fn current_pattern_id(&self) -> PatternID {}
    pub fn pattern_len(&self) -> usize {}
    pub fn add_empty(&mut self) -> Result<StateID, BuildError> {}
    pub fn add_union(
        &mut self,
        alternates: Vec<StateID>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_union_reverse(
        &mut self,
        alternates: Vec<StateID>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_range(&mut self, trans: Transition) -> Result<StateID, BuildError> {}
    pub fn add_sparse(
        &mut self,
        transitions: Vec<Transition>,
    ) -> Result<StateID, BuildError> {}
    pub fn add_look(
        &mut self,
        next: StateID,
        look: Look,
    ) -> Result<StateID, BuildError> {}
    pub fn add_capture_start(
        &mut self,
        next: StateID,
        group_index: u32,
        name: Option<Arc<str>>,
    ) -> Result<StateID, BuildError> {
        let pid = self.current_pattern_id();
        let group_index = match SmallIndex::try_from(group_index) {
            Err(_) => return Err(BuildError::invalid_capture_index(group_index)),
            Ok(group_index) => group_index,
        };
        if pid.as_usize() >= self.captures.len() {
            for _ in 0..=(pid.as_usize() - self.captures.len()) {
                self.captures.push(vec![]);
            }
        }
        if group_index.as_usize() >= self.captures[pid].len() {
            for _ in 0..(group_index.as_usize() - self.captures[pid].len()) {
                self.captures[pid].push(None);
            }
            self.captures[pid].push(name);
        }
        self.add(State::CaptureStart {
            pattern_id: pid,
            group_index,
            next,
        })
    }
    pub fn add_capture_end(
        &mut self,
        next: StateID,
        group_index: u32,
    ) -> Result<StateID, BuildError> {
        let pid = self.current_pattern_id();
        let group_index = match SmallIndex::try_from(group_index) {
            Err(_) => return Err(BuildError::invalid_capture_index(group_index)),
            Ok(group_index) => group_index,
        };
        self.add(State::CaptureEnd {
            pattern_id: pid,
            group_index,
            next,
        })
    }
    pub fn add_fail(&mut self) -> Result<StateID, BuildError> {}
    pub fn add_match(&mut self) -> Result<StateID, BuildError> {
        let pattern_id = self.current_pattern_id();
        let sid = self.add(State::Match { pattern_id })?;
        Ok(sid)
    }
    fn add(&mut self, state: State) -> Result<StateID, BuildError> {}
    pub fn patch(&mut self, from: StateID, to: StateID) -> Result<(), BuildError> {
        let old_memory_states = self.memory_states;
        match self.states[from] {
            State::Empty { ref mut next } => {
                *next = to;
            }
            State::ByteRange { ref mut trans } => {
                trans.next = to;
            }
            State::Sparse { .. } => panic!("cannot patch from a sparse NFA state"),
            State::Look { ref mut next, .. } => {
                *next = to;
            }
            State::Union { ref mut alternates } => {
                alternates.push(to);
                self.memory_states += mem::size_of::<StateID>();
            }
            State::UnionReverse { ref mut alternates } => {
                alternates.push(to);
                self.memory_states += mem::size_of::<StateID>();
            }
            State::CaptureStart { ref mut next, .. } => {
                *next = to;
            }
            State::CaptureEnd { ref mut next, .. } => {
                *next = to;
            }
            State::Fail => {}
            State::Match { .. } => {}
        }
        if old_memory_states != self.memory_states {
            self.check_size_limit()?;
        }
        Ok(())
    }
    pub fn set_utf8(&mut self, yes: bool) {}
    pub fn get_utf8(&self) -> bool {}
    pub fn set_reverse(&mut self, yes: bool) {}
    pub fn get_reverse(&self) -> bool {}
    pub fn set_look_matcher(&mut self, m: LookMatcher) {}
    pub fn get_look_matcher(&self) -> &LookMatcher {}
    pub fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError> {}
    pub fn get_size_limit(&self) -> Option<usize> {}
    pub fn memory_usage(&self) -> usize {}
    fn check_size_limit(&self) -> Result<(), BuildError> {}
}

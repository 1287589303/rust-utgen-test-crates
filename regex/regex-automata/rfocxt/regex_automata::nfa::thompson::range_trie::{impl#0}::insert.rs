use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
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
#[derive(Clone, Debug, Eq, PartialEq)]
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
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
struct State {
    /// A sorted sequence of non-overlapping transitions to other states. Each
    /// transition corresponds to a single range of bytes.
    transitions: Vec<Transition>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Default)]
struct State {
    transitions: Vec<Transition>,
    chunks: Vec<(usize, usize)>,
}
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone)]
struct State<'a> {
    /// The identifier of this state.
    id: StateID,
    /// Whether this is a match state or not.
    is_match: bool,
    /// The number of transitions in this state.
    ntrans: usize,
    /// Pairs of input ranges, where there is one pair for each transition.
    /// Each pair specifies an inclusive start and end byte range for the
    /// corresponding transition.
    input_ranges: &'a [u8],
    /// Transitions to the next state. This slice contains native endian
    /// encoded state identifiers, with `S` as the representation. Thus, there
    /// are `ntrans * size_of::<S>()` bytes in this slice.
    next: &'a [u8],
    /// If this is a match state, then this contains the pattern IDs that match
    /// when the DFA is in this state.
    ///
    /// This is a contiguous sequence of 32-bit native endian encoded integers.
    pattern_ids: &'a [u8],
    /// An accelerator for this state, if present. If this state has no
    /// accelerator, then this is an empty slice. When non-empty, this slice
    /// has length at most 3 and corresponds to the exhaustive set of bytes
    /// that must be seen in order to transition out of this state.
    accel: &'a [u8],
}
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[derive(Clone, Debug)]
struct NextDupe {
    /// The state we want to duplicate.
    old_id: StateID,
    /// The ID of the new state that is a duplicate of old_id.
    new_id: StateID,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Debug)]
struct NextIter {
    state_id: StateID,
    tidx: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SplitRange {
    Old(Utf8Range),
    New(Utf8Range),
    Both(Utf8Range),
}
#[derive(Clone, Eq, PartialEq)]
pub enum State {
    /// A state with a single transition that can only be taken if the current
    /// input symbol is in a particular range of bytes.
    ByteRange {
        /// The transition from this state to the next.
        trans: Transition,
    },
    /// A state with possibly many transitions represented in a sparse fashion.
    /// Transitions are non-overlapping and ordered lexicographically by input
    /// range.
    ///
    /// In practice, this is used for encoding UTF-8 automata. Its presence is
    /// primarily an optimization that avoids many additional unconditional
    /// epsilon transitions (via [`Union`](State::Union) states), and thus
    /// decreases the overhead of traversing the NFA. This can improve both
    /// matching time and DFA construction time.
    Sparse(SparseTransitions),
    /// A dense representation of a state with multiple transitions.
    Dense(DenseTransitions),
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around. Look-around is limited to anchor and word boundary
    /// assertions.
    ///
    /// Look-around states are meant to be evaluated while performing epsilon
    /// closure (computing the set of states reachable from a particular state
    /// via only epsilon transitions). If the current position in the haystack
    /// satisfies the look-around assertion, then you're permitted to follow
    /// that epsilon transition.
    Look {
        /// The look-around assertion that must be satisfied before moving
        /// to `next`.
        look: Look,
        /// The state to transition to if the look-around assertion is
        /// satisfied.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union {
        /// An ordered sequence of unconditional epsilon transitions to other
        /// states. Transitions earlier in the sequence are preferred over
        /// transitions later in the sequence.
        alternates: Box<[StateID]>,
    },
    /// An alternation such that there exists precisely two unconditional
    /// epsilon transitions, where matches found via `alt1` are preferred over
    /// matches found via `alt2`.
    ///
    /// This state exists as a common special case of Union where there are
    /// only two alternates. In this case, we don't need any allocations to
    /// represent the state. This saves a bit of memory and also saves an
    /// additional memory access when traversing the NFA.
    BinaryUnion {
        /// An unconditional epsilon transition to another NFA state. This
        /// is preferred over `alt2`.
        alt1: StateID,
        /// An unconditional epsilon transition to another NFA state. Matches
        /// reported via this transition should only be reported if no matches
        /// were found by following `alt1`.
        alt2: StateID,
    },
    /// An empty state that records a capture location.
    ///
    /// From the perspective of finite automata, this is precisely equivalent
    /// to an unconditional epsilon transition, but serves the purpose of
    /// instructing NFA simulations to record additional state when the finite
    /// state machine passes through this epsilon transition.
    ///
    /// `slot` in this context refers to the specific capture group slot
    /// offset that is being recorded. Each capturing group has two slots
    /// corresponding to the start and end of the matching portion of that
    /// group.
    ///
    /// The pattern ID and capture group index are also included in this state
    /// in case they are useful. But mostly, all you'll need is `next` and
    /// `slot`.
    Capture {
        /// The state to transition to, unconditionally.
        next: StateID,
        /// The pattern ID that this capture belongs to.
        pattern_id: PatternID,
        /// The capture group index that this capture belongs to. Capture group
        /// indices are local to each pattern. For example, when capturing
        /// groups are enabled, every pattern has a capture group at index
        /// `0`.
        group_index: SmallIndex,
        /// The slot index for this capture. Every capturing group has two
        /// slots: one for the start haystack offset and one for the end
        /// haystack offset. Unlike capture group indices, slot indices are
        /// global across all patterns in this NFA. That is, each slot belongs
        /// to a single pattern, but there is only one slot at index `i`.
        slot: SmallIndex,
    },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose
    /// a `Fail` state to represent them. (An empty character class can be
    /// thought of as an empty set. Since nothing is in an empty set, they can
    /// never match anything.)
    Fail,
    /// A match state. There is at least one such occurrence of this state for
    /// each regex that can match that is in this NFA.
    Match {
        /// The matching pattern ID.
        pattern_id: PatternID,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum State {
    /// An empty state whose only purpose is to forward the automaton to
    /// another state via an unconditional epsilon transition.
    ///
    /// Unconditional epsilon transitions are quite useful during the
    /// construction of an NFA, as they permit the insertion of no-op
    /// placeholders that make it easier to compose NFA sub-graphs. When
    /// the Thompson NFA builder produces a final NFA, all unconditional
    /// epsilon transitions are removed, and state identifiers are remapped
    /// accordingly.
    Empty {
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// A state that only transitions to another state if the current input
    /// byte is in a particular range of bytes.
    ByteRange { trans: Transition },
    /// A state with possibly many transitions, represented in a sparse
    /// fashion. Transitions must be ordered lexicographically by input range
    /// and be non-overlapping. As such, this may only be used when every
    /// transition has equal priority. (In practice, this is only used for
    /// encoding large UTF-8 automata.) In contrast, a `Union` state has each
    /// alternate in order of priority. Priority is used to implement greedy
    /// matching and also alternations themselves, e.g., `abc|a` where `abc`
    /// has priority over `a`.
    ///
    /// To clarify, it is possible to remove `Sparse` and represent all things
    /// that `Sparse` is used for via `Union`. But this creates a more bloated
    /// NFA with more epsilon transitions than is necessary in the special case
    /// of character classes.
    Sparse { transitions: Vec<Transition> },
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around.
    Look { look: Look, next: StateID },
    /// An empty state that records the start of a capture location. This is an
    /// unconditional epsilon transition like `Empty`, except it can be used to
    /// record position information for a capture group when using the NFA for
    /// search.
    CaptureStart {
        /// The ID of the pattern that this capture was defined.
        pattern_id: PatternID,
        /// The capture group index that this capture state corresponds to.
        /// The capture group index is always relative to its corresponding
        /// pattern. Therefore, in the presence of multiple patterns, both the
        /// pattern ID and the capture group index are required to uniquely
        /// identify a capturing group.
        group_index: SmallIndex,
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// An empty state that records the end of a capture location. This is an
    /// unconditional epsilon transition like `Empty`, except it can be used to
    /// record position information for a capture group when using the NFA for
    /// search.
    CaptureEnd {
        /// The ID of the pattern that this capture was defined.
        pattern_id: PatternID,
        /// The capture group index that this capture state corresponds to.
        /// The capture group index is always relative to its corresponding
        /// pattern. Therefore, in the presence of multiple patterns, both the
        /// pattern ID and the capture group index are required to uniquely
        /// identify a capturing group.
        group_index: SmallIndex,
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union { alternates: Vec<StateID> },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via later transitions are
    /// preferred over earlier transitions.
    ///
    /// This "reverse" state exists for convenience during compilation that
    /// permits easy construction of non-greedy combinations of NFA states. At
    /// the end of compilation, Union and UnionReverse states are merged into
    /// one Union type of state, where the latter has its epsilon transitions
    /// reversed to reflect the priority inversion.
    ///
    /// The "convenience" here arises from the fact that as new states are
    /// added to the list of `alternates`, we would like that add operation
    /// to be amortized constant time. But if we used a `Union`, we'd need to
    /// prepend the state, which takes O(n) time. There are other approaches we
    /// could use to solve this, but this seems simple enough.
    UnionReverse { alternates: Vec<StateID> },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose a
    /// `Fail` state to represent it.
    Fail,
    /// A match state. There is at most one such occurrence of this state in
    /// an NFA for each pattern compiled into the NFA. At time of writing, a
    /// match state is always produced for every pattern given, but in theory,
    /// if a pattern can never lead to a match, then the match state could be
    /// omitted.
    ///
    /// `pattern_id` refers to the ID of the pattern itself, which corresponds
    /// to the pattern's index (starting at 0).
    Match { pattern_id: PatternID },
}
impl RangeTrie {
    pub fn new() -> RangeTrie {}
    pub fn clear(&mut self) {}
    pub fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {}
    pub fn insert(&mut self, ranges: &[Utf8Range]) {
        assert!(! ranges.is_empty());
        assert!(ranges.len() <= 4);
        let mut stack = mem::replace(&mut self.insert_stack, vec![]);
        stack.clear();
        stack.push(NextInsert::new(ROOT, ranges));
        while let Some(next) = stack.pop() {
            let (state_id, ranges) = (next.state_id(), next.ranges());
            assert!(! ranges.is_empty());
            let (mut new, rest) = (ranges[0], &ranges[1..]);
            let mut i = self.state(state_id).find(new);
            if i == self.state(state_id).transitions.len() {
                let next_id = NextInsert::push(self, &mut stack, rest);
                self.add_transition(state_id, new, next_id);
                continue;
            }
            'OUTER: loop {
                let old = self.state(state_id).transitions[i].clone();
                let split = match Split::new(old.range, new) {
                    Some(split) => split,
                    None => {
                        let next_id = NextInsert::push(self, &mut stack, rest);
                        self.add_transition_at(i, state_id, new, next_id);
                        continue;
                    }
                };
                let splits = split.as_slice();
                if splits.len() == 1 {
                    if !rest.is_empty() {
                        stack.push(NextInsert::new(old.next_id, rest));
                    }
                    break;
                }
                let mut first = true;
                let mut add_trans = |trie: &mut RangeTrie, pos, from, range, to| {
                    if first {
                        trie.set_transition_at(pos, from, range, to);
                        first = false;
                    } else {
                        trie.add_transition_at(pos, from, range, to);
                    }
                };
                for (j, &srange) in splits.iter().enumerate() {
                    match srange {
                        SplitRange::Old(r) => {
                            let dup_id = self.duplicate(old.next_id);
                            add_trans(self, i, state_id, r, dup_id);
                        }
                        SplitRange::New(r) => {
                            {
                                let trans = &self.state(state_id).transitions;
                                if j + 1 == splits.len() && i < trans.len()
                                    && intersects(r, trans[i].range)
                                {
                                    new = r;
                                    continue 'OUTER;
                                }
                            }
                            let next_id = NextInsert::push(self, &mut stack, rest);
                            add_trans(self, i, state_id, r, next_id);
                        }
                        SplitRange::Both(r) => {
                            if !rest.is_empty() {
                                stack.push(NextInsert::new(old.next_id, rest));
                            }
                            add_trans(self, i, state_id, r, old.next_id);
                        }
                    }
                    i += 1;
                }
                break;
            }
        }
        self.insert_stack = stack;
    }
    pub fn add_empty(&mut self) -> StateID {}
    fn duplicate(&mut self, old_id: StateID) -> StateID {
        if old_id == FINAL {
            return FINAL;
        }
        let mut stack = mem::replace(&mut self.dupe_stack, vec![]);
        stack.clear();
        let new_id = self.add_empty();
        stack.push(NextDupe { old_id, new_id });
        while let Some(NextDupe { old_id, new_id }) = stack.pop() {
            for i in 0..self.state(old_id).transitions.len() {
                let t = self.state(old_id).transitions[i].clone();
                if t.next_id == FINAL {
                    self.add_transition(new_id, t.range, FINAL);
                    continue;
                }
                let new_child_id = self.add_empty();
                self.add_transition(new_id, t.range, new_child_id);
                stack
                    .push(NextDupe {
                        old_id: t.next_id,
                        new_id: new_child_id,
                    });
            }
        }
        self.dupe_stack = stack;
        new_id
    }
    fn add_transition(&mut self, from_id: StateID, range: Utf8Range, next_id: StateID) {
        self.state_mut(from_id).transitions.push(Transition { range, next_id });
    }
    fn add_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id).transitions.insert(i, Transition { range, next_id });
    }
    fn set_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {}
    fn state(&self, id: StateID) -> &State {
        &self.states[id]
    }
    fn state_mut(&mut self, id: StateID) -> &mut State {}
}
impl Split {
    fn new(o: Utf8Range, n: Utf8Range) -> Option<Split> {
        let range = |r: RangeInclusive<u8>| Utf8Range {
            start: *r.start(),
            end: *r.end(),
        };
        let old = |r| SplitRange::Old(range(r));
        let new = |r| SplitRange::New(range(r));
        let both = |r| SplitRange::Both(range(r));
        let (a, b, x, y) = (o.start, o.end, n.start, n.end);
        if b < x || y < a {
            None
        } else if a == x && b == y {
            Some(Split::parts1(both(a..=b)))
        } else if a == x && b < y {
            Some(Split::parts2(both(a..=b), new(b + 1..=y)))
        } else if b == y && a > x {
            Some(Split::parts2(new(x..=a - 1), both(a..=b)))
        } else if x == a && y < b {
            Some(Split::parts2(both(x..=y), old(y + 1..=b)))
        } else if y == b && x > a {
            Some(Split::parts2(old(a..=x - 1), both(x..=y)))
        } else if a > x && b < y {
            Some(Split::parts3(new(x..=a - 1), both(a..=b), new(b + 1..=y)))
        } else if x > a && y < b {
            Some(Split::parts3(old(a..=x - 1), both(x..=y), old(y + 1..=b)))
        } else if b == x && a < y {
            Some(Split::parts3(old(a..=b - 1), both(b..=b), new(b + 1..=y)))
        } else if y == a && x < b {
            Some(Split::parts3(new(x..=y - 1), both(y..=y), old(y + 1..=b)))
        } else if b > x && b < y {
            Some(Split::parts3(old(a..=x - 1), both(x..=b), new(b + 1..=y)))
        } else if y > a && y < b {
            Some(Split::parts3(new(x..=a - 1), both(a..=y), old(y + 1..=b)))
        } else {
            unreachable!()
        }
    }
    fn parts1(r1: SplitRange) -> Split {}
    fn parts2(r1: SplitRange, r2: SplitRange) -> Split {}
    fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split {}
    fn as_slice(&self) -> &[SplitRange] {
        &self.partitions[..self.len]
    }
}
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
    fn state_id(&self) -> StateID {
        self.state_id
    }
    fn ranges(&self) -> &[Utf8Range] {
        &self.ranges[..usize::try_from(self.len).unwrap()]
    }
}
impl State {
    fn find(&self, range: Utf8Range) -> usize {
        /// Returns the position `i` at which `pred(xs[i])` first returns true
        /// such that for all `j >= i`, `pred(xs[j]) == true`. If `pred` never
        /// returns true, then `xs.len()` is returned.
        ///
        /// We roll our own binary search because it doesn't seem like the
        /// standard library's binary search can be used here. Namely, if
        /// there is an overlapping range, then we want to find the first such
        /// occurrence, but there may be many. Or at least, it's not quite
        /// clear to me how to do it.
        fn binary_search<T, F>(xs: &[T], mut pred: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            let (mut left, mut right) = (0, xs.len());
            while left < right {
                let mid = (left + right) / 2;
                if pred(&xs[mid]) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
        binary_search(&self.transitions, |t| range.start <= t.range.end)
    }
    fn clear(&mut self) {}
}
fn intersects(r1: Utf8Range, r2: Utf8Range) -> bool {
    !(r1.end < r2.start || r2.end < r1.start)
}

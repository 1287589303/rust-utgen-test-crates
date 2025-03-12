fn minimum_cache_capacity(
    nfa: &thompson::NFA,
    classes: &ByteClasses,
    starts_for_each_pattern: bool,
) -> usize {
    const ID_SIZE: usize = size_of::<LazyStateID>();
    const STATE_SIZE: usize = size_of::<State>();

    let stride = 1 << classes.stride2();
    let states_len = nfa.states().len();
    let sparses = 2 * states_len * NFAStateID::SIZE;
    let trans = MIN_STATES * stride * ID_SIZE;

    let mut starts = Start::len() * ID_SIZE;
    if starts_for_each_pattern {
        starts += (Start::len() * nfa.pattern_len()) * ID_SIZE;
    }

    // The min number of states HAS to be at least 4: we have 3 sentinel states
    // and then we need space for one more when we save a state after clearing
    // the cache. We also need space for one more, otherwise we get stuck in a
    // loop where we try to add a 5th state, which gets rejected, which clears
    // the cache, which adds back a saved state (4th total state) which then
    // tries to add the 5th state again.
    assert!(MIN_STATES >= 5, "minimum number of states has to be at least 5");
    // The minimum number of non-sentinel states. We consider this separately
    // because sentinel states are much smaller in that they contain no NFA
    // states. Given our aggressive calculation here, it's worth being more
    // precise with the number of states we need.
    let non_sentinel = MIN_STATES.checked_sub(SENTINEL_STATES).unwrap();

    // Every `State` has 5 bytes for flags, 4 bytes (max) for the number of
    // patterns, followed by 32-bit encodings of patterns and then delta
    // varint encodings of NFA state IDs. We use the worst case (which isn't
    // technically possible) of 5 bytes for each NFA state ID.
    //
    // HOWEVER, three of the states needed by a lazy DFA are just the sentinel
    // unknown, dead and quit states. Those states have a known size and it is
    // small.
    let dead_state_size = State::dead().memory_usage();
    let max_state_size = 5 + 4 + (nfa.pattern_len() * 4) + (states_len * 5);
    let states = (SENTINEL_STATES * (STATE_SIZE + dead_state_size))
        + (non_sentinel * (STATE_SIZE + max_state_size));
    // NOTE: We don't double count heap memory used by State for this map since
    // we use reference counting to avoid doubling memory usage. (This tends to
    // be where most memory is allocated in the cache.)
    let states_to_sid = (MIN_STATES * STATE_SIZE) + (MIN_STATES * ID_SIZE);
    let stack = states_len * NFAStateID::SIZE;
    let scratch_state_builder = max_state_size;

    trans
        + starts
        + states
        + states_to_sid
        + sparses
        + stack
        + scratch_state_builder
}
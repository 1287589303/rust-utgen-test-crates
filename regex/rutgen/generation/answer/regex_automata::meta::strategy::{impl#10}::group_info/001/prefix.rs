// Answer 0

#[test]
fn test_group_info_valid() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner {}));
    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA(Arc::new(Inner {})),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let reverse_inner = ReverseInner {
        core,
        preinner: Prefilter {
            pre: Arc::new(PrefilterI::default()),
            is_fast: true,
            max_needle_len: 100,
        },
        nfarev: NFA(Arc::new(Inner {})),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let _ = reverse_inner.group_info();
}

#[test]
fn test_group_info_memory_usage() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner {}));
    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA(Arc::new(Inner {})),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let reverse_inner = ReverseInner {
        core,
        preinner: Prefilter {
            pre: Arc::new(PrefilterI::default()),
            is_fast: true,
            max_needle_len: 50,
        },
        nfarev: NFA(Arc::new(Inner {})),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let _ = reverse_inner.group_info();
}


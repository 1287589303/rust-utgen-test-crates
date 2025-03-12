use core::{cell::RefCell, fmt, mem};
use alloc::{collections::BTreeMap, rc::Rc, vec, vec::Vec};
use crate::{
    dfa::{automaton::Automaton, dense, DEAD},
    util::{alphabet, primitives::{PatternID, StateID}},
};
pub(crate) struct Minimizer<'a> {
    dfa: &'a mut dense::OwnedDFA,
    in_transitions: Vec<Vec<Vec<StateID>>>,
    partitions: Vec<StateSet>,
    waiting: Vec<StateSet>,
}
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct StateSet {
    ids: Rc<RefCell<Vec<StateID>>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> Minimizer<'a> {
    pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {
        let in_transitions = Minimizer::incoming_transitions(dfa);
        let partitions = Minimizer::initial_partitions(dfa);
        let waiting = partitions.clone();
        Minimizer {
            dfa,
            in_transitions,
            partitions,
            waiting,
        }
    }
    pub fn run(mut self) {}
    fn find_waiting(&self, set: &StateSet) -> Option<usize> {}
    fn find_incoming_to(
        &self,
        b: alphabet::Unit,
        set: &StateSet,
        incoming: &mut StateSet,
    ) {}
    fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {
        let mut matching: BTreeMap<Vec<PatternID>, StateSet> = BTreeMap::new();
        let mut is_quit = StateSet::empty();
        let mut no_match = StateSet::empty();
        for state in dfa.states() {
            if dfa.is_match_state(state.id()) {
                let mut pids = vec![];
                for i in 0..dfa.match_len(state.id()) {
                    pids.push(dfa.match_pattern(state.id(), i));
                }
                matching.entry(pids).or_insert(StateSet::empty()).add(state.id());
            } else if dfa.is_quit_state(state.id()) {
                is_quit.add(state.id());
            } else {
                no_match.add(state.id());
            }
        }
        let mut sets: Vec<StateSet> = matching.into_iter().map(|(_, set)| set).collect();
        sets.push(no_match);
        sets.push(is_quit);
        sets
    }
    fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {
        let mut incoming = vec![];
        for _ in dfa.states() {
            incoming.push(vec![vec![]; dfa.alphabet_len()]);
        }
        for state in dfa.states() {
            for (b, next) in state.transitions() {
                incoming[dfa.to_index(next)][b.as_usize()].push(state.id());
            }
        }
        incoming
    }
}

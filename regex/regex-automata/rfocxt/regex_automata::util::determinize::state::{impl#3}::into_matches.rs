use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {}
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {
        self.0.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);
        StateBuilderMatches(self.0)
    }
    fn clear(&mut self) {}
    pub(crate) fn capacity(&self) -> usize {}
}

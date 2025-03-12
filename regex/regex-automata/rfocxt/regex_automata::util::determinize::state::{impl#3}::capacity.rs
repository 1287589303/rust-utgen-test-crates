use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {}
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {}
    fn clear(&mut self) {}
    pub(crate) fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

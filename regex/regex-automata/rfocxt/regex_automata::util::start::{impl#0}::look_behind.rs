use crate::util::{
    look::LookMatcher, search::{Anchored, Input},
    wire::{self, DeserializeError, SerializeError},
};
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
impl Config {
    pub fn new() -> Config {}
    pub fn from_input_forward(input: &Input<'_>) -> Config {}
    pub fn from_input_reverse(input: &Input<'_>) -> Config {}
    pub fn look_behind(mut self, byte: Option<u8>) -> Config {
        self.look_behind = byte;
        self
    }
    pub fn anchored(mut self, mode: Anchored) -> Config {}
    pub fn get_look_behind(&self) -> Option<u8> {}
    pub fn get_anchored(&self) -> Anchored {}
}

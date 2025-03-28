type Range = &'static [(char, char)];
type PropertyValues = &'static [(&'static str, &'static str)];
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::hir;
#[derive(Debug)]
pub struct SimpleCaseFolder {
    /// The simple case fold table. It's a sorted association list, where the
    /// keys are Unicode scalar values and the values are the corresponding
    /// equivalence class (not including the key) of the "simple" case folded
    /// Unicode scalar values.
    table: &'static [(char, &'static [char])],
    /// The last codepoint that was used for a lookup.
    last: Option<char>,
    /// The index to the entry in `table` corresponding to the smallest key `k`
    /// such that `k > k0`, where `k0` is the most recent key lookup. Note that
    /// in particular, `k0` may not be in the table!
    next: usize,
}
impl SimpleCaseFolder {
    pub fn new() -> Result<SimpleCaseFolder, CaseFoldError> {}
    pub fn mapping(&mut self, c: char) -> &'static [char] {}
    pub fn overlaps(&self, start: char, end: char) -> bool {
        use core::cmp::Ordering;
        assert!(start <= end);
        self.table
            .binary_search_by(|&(c, _)| {
                if start <= c && c <= end {
                    Ordering::Equal
                } else if c > end {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .is_ok()
    }
    fn get(&self, c: char) -> Result<usize, usize> {}
}

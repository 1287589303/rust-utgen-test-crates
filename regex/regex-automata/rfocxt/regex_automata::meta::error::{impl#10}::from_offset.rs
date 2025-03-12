use regex_syntax::{ast, hir};
use crate::{nfa, util::search::MatchError, PatternID};
#[derive(Debug)]
pub(crate) struct RetryFailError {
    offset: usize,
}
impl RetryFailError {
    pub(crate) fn from_offset(offset: usize) -> RetryFailError {
        RetryFailError { offset }
    }
}

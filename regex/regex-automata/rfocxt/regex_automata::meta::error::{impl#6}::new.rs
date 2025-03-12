use regex_syntax::{ast, hir};
use crate::{nfa, util::search::MatchError, PatternID};
#[derive(Debug)]
pub(crate) struct RetryQuadraticError(());
impl RetryQuadraticError {
    pub(crate) fn new() -> RetryQuadraticError {
        RetryQuadraticError(())
    }
}

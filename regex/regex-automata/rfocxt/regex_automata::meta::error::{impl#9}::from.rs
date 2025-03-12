use regex_syntax::{ast, hir};
use crate::{nfa, util::search::MatchError, PatternID};
#[derive(Debug)]
pub(crate) struct RetryFailError {
    offset: usize,
}
#[derive(Debug)]
pub(crate) struct RetryQuadraticError(());
#[derive(Debug)]
pub(crate) enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
impl From<RetryFailError> for RetryError {
    fn from(err: RetryFailError) -> RetryError {
        RetryError::Fail(err)
    }
}

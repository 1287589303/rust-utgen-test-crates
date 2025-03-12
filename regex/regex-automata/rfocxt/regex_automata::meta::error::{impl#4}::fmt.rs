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
impl core::fmt::Display for RetryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            RetryError::Quadratic(ref err) => err.fmt(f),
            RetryError::Fail(ref err) => err.fmt(f),
        }
    }
}
impl core::fmt::Display for RetryFailError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "regex engine failed at offset {:?}", self.offset)
    }
}
impl core::fmt::Display for RetryQuadraticError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "regex engine gave up to avoid quadratic behavior")
    }
}

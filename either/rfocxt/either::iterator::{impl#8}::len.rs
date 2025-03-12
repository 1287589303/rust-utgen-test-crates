use super::{for_both, Either, Left, Right};
use core::iter;
#[derive(Clone, Debug)]
pub struct IterEither<L, R> {
    inner: Either<L, R>,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
    Right(R),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> ExactSizeIterator for IterEither<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator,
{
    fn len(&self) -> usize {
        for_both!(self.inner, ref inner => inner.len())
    }
}

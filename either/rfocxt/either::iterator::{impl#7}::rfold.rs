use super::{for_both, Either, Left, Right};
use core::iter;
#[derive(Clone, Debug)]
pub struct IterEither<L, R> {
    inner: Either<L, R>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
    Right(R),
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> DoubleEndedIterator for IterEither<L, R>
where
    L: DoubleEndedIterator,
    R: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(map_either!(self.inner, ref mut inner => inner.next_back() ?))
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(map_either!(self.inner, ref mut inner => inner.nth_back(n) ?))
    }
    fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        wrap_either!(self.inner => .rfold(init, f))
    }
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        wrap_either!(& mut self.inner => .rfind(predicate))
    }
}

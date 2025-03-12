use super::{Either, Left, Right};
pub trait IntoEither: Sized {
    fn into_either(self, into_left: bool) -> Either<Self, Self> {
        if into_left { Left(self) } else { Right(self) }
    }
    fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        let into_left = into_left(&self);
        self.into_either(into_left)
    }
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

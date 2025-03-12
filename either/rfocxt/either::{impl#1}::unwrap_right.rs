use core::convert::{AsMut, AsRef};
use core::fmt;
use core::future::Future;
use core::ops::Deref;
use core::ops::DerefMut;
use core::pin::Pin;
#[cfg(any(test, feature = "std"))]
use std::error::Error;
#[cfg(any(test, feature = "std"))]
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
pub use crate::Either::{Left, Right};
pub use self::iterator::IterEither;
pub use self::into_either::IntoEither;
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
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {}
    pub fn is_right(&self) -> bool {}
    pub fn left(self) -> Option<L> {}
    pub fn right(self) -> Option<R> {}
    pub fn as_ref(&self) -> Either<&L, &R> {}
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {}
    pub fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&L>, Pin<&R>> {}
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut L>, Pin<&mut R>> {}
    pub fn flip(self) -> Either<R, L> {}
    pub fn map_left<F, M>(self, f: F) -> Either<M, R>
    where
        F: FnOnce(L) -> M,
    {}
    pub fn map_right<F, S>(self, f: F) -> Either<L, S>
    where
        F: FnOnce(R) -> S,
    {}
    pub fn map_either<F, G, M, S>(self, f: F, g: G) -> Either<M, S>
    where
        F: FnOnce(L) -> M,
        G: FnOnce(R) -> S,
    {}
    pub fn map_either_with<Ctx, F, G, M, S>(self, ctx: Ctx, f: F, g: G) -> Either<M, S>
    where
        F: FnOnce(Ctx, L) -> M,
        G: FnOnce(Ctx, R) -> S,
    {}
    pub fn either<F, G, T>(self, f: F, g: G) -> T
    where
        F: FnOnce(L) -> T,
        G: FnOnce(R) -> T,
    {}
    pub fn either_with<Ctx, F, G, T>(self, ctx: Ctx, f: F, g: G) -> T
    where
        F: FnOnce(Ctx, L) -> T,
        G: FnOnce(Ctx, R) -> T,
    {}
    pub fn left_and_then<F, S>(self, f: F) -> Either<S, R>
    where
        F: FnOnce(L) -> Either<S, R>,
    {}
    pub fn right_and_then<F, S>(self, f: F) -> Either<L, S>
    where
        F: FnOnce(R) -> Either<L, S>,
    {}
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> Either<L::IntoIter, R::IntoIter>
    where
        L: IntoIterator,
        R: IntoIterator<Item = L::Item>,
    {}
    pub fn iter(
        &self,
    ) -> Either<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
    where
        for<'a> &'a L: IntoIterator,
        for<'a> &'a R: IntoIterator<Item = <&'a L as IntoIterator>::Item>,
    {}
    pub fn iter_mut(
        &mut self,
    ) -> Either<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>
    where
        for<'a> &'a mut L: IntoIterator,
        for<'a> &'a mut R: IntoIterator<Item = <&'a mut L as IntoIterator>::Item>,
    {}
    pub fn factor_into_iter(self) -> IterEither<L::IntoIter, R::IntoIter>
    where
        L: IntoIterator,
        R: IntoIterator,
    {}
    pub fn factor_iter(
        &self,
    ) -> IterEither<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
    where
        for<'a> &'a L: IntoIterator,
        for<'a> &'a R: IntoIterator,
    {}
    pub fn factor_iter_mut(
        &mut self,
    ) -> IterEither<
        <&mut L as IntoIterator>::IntoIter,
        <&mut R as IntoIterator>::IntoIter,
    >
    where
        for<'a> &'a mut L: IntoIterator,
        for<'a> &'a mut R: IntoIterator,
    {}
    pub fn left_or(self, other: L) -> L {}
    pub fn left_or_default(self) -> L
    where
        L: Default,
    {}
    pub fn left_or_else<F>(self, f: F) -> L
    where
        F: FnOnce(R) -> L,
    {}
    pub fn right_or(self, other: R) -> R {}
    pub fn right_or_default(self) -> R
    where
        R: Default,
    {}
    pub fn right_or_else<F>(self, f: F) -> R
    where
        F: FnOnce(L) -> R,
    {}
    pub fn unwrap_left(self) -> L
    where
        R: core::fmt::Debug,
    {}
    pub fn unwrap_right(self) -> R
    where
        L: core::fmt::Debug,
    {
        match self {
            Either::Right(r) => r,
            Either::Left(l) => {
                panic!("called `Either::unwrap_right()` on a `Left` value: {:?}", l)
            }
        }
    }
    pub fn expect_left(self, msg: &str) -> L
    where
        R: core::fmt::Debug,
    {}
    pub fn expect_right(self, msg: &str) -> R
    where
        L: core::fmt::Debug,
    {}
    pub fn either_into<T>(self) -> T
    where
        L: Into<T>,
        R: Into<T>,
    {}
}

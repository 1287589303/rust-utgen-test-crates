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
impl<L, R> Either<&mut L, &mut R> {
    pub fn cloned(self) -> Either<L, R>
    where
        L: Clone,
        R: Clone,
    {}
    pub fn copied(self) -> Either<L, R>
    where
        L: Copy,
        R: Copy,
    {
        map_either!(self, inner => * inner)
    }
}

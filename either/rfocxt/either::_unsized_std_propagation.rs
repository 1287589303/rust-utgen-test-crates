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
#[cfg(feature = "std")]
fn _unsized_std_propagation() {
    check_t!(::std::path::Path);
    check_t!(::std::ffi::OsStr);
    check_t!(::std::ffi::CStr);
}

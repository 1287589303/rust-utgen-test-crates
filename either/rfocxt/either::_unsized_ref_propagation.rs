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
fn _unsized_ref_propagation() {
    check_t!(str);
    fn check_array_ref<T: AsRef<[Item]>, Item>() {}
    fn check_array_mut<T: AsMut<[Item]>, Item>() {}
    fn propagate_array_ref<T1: AsRef<[Item]>, T2: AsRef<[Item]>, Item>() {
        check_array_ref::<Either<T1, T2>, _>()
    }
    fn propagate_array_mut<T1: AsMut<[Item]>, T2: AsMut<[Item]>, Item>() {
        check_array_mut::<Either<T1, T2>, _>()
    }
}

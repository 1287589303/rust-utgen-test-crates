use crate::buf::{limit, Chain, Limit, UninitSlice};
#[cfg(feature = "std")]
use crate::buf::{writer, Writer};
use crate::{panic_advance, panic_does_not_fit, TryGetError};
use core::{mem, ptr, usize};
use alloc::{boxed::Box, vec::Vec};
fn _assert_trait_object(_b: &dyn BufMut) {}

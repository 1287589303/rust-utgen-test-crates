#[cfg(feature = "std")]
use crate::buf::{reader, Reader};
use crate::buf::{take, Chain, Take};
#[cfg(feature = "std")]
use crate::{min_u64_usize, saturating_sub_usize_u64};
use crate::{panic_advance, panic_does_not_fit, TryGetError};
#[cfg(feature = "std")]
use std::io::IoSlice;
use alloc::boxed::Box;
pub trait Buf {
    fn remaining(&self) -> usize;
    #[cfg_attr(docsrs, doc(alias = "bytes"))]
    fn chunk(&self) -> &[u8];
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize;
    fn advance(&mut self, cnt: usize);
    fn has_remaining(&self) -> bool;
    fn copy_to_slice(&mut self, dst: &mut [u8]);
    fn get_u8(&mut self) -> u8;
    fn get_i8(&mut self) -> i8;
    fn get_u16(&mut self) -> u16;
    fn get_u16_le(&mut self) -> u16;
    fn get_u16_ne(&mut self) -> u16;
    fn get_i16(&mut self) -> i16;
    fn get_i16_le(&mut self) -> i16;
    fn get_i16_ne(&mut self) -> i16;
    fn get_u32(&mut self) -> u32;
    fn get_u32_le(&mut self) -> u32;
    fn get_u32_ne(&mut self) -> u32;
    fn get_i32(&mut self) -> i32;
    fn get_i32_le(&mut self) -> i32;
    fn get_i32_ne(&mut self) -> i32;
    fn get_u64(&mut self) -> u64;
    fn get_u64_le(&mut self) -> u64;
    fn get_u64_ne(&mut self) -> u64;
    fn get_i64(&mut self) -> i64;
    fn get_i64_le(&mut self) -> i64;
    fn get_i64_ne(&mut self) -> i64;
    fn get_u128(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_be_bytes);
    }
    fn get_u128_le(&mut self) -> u128;
    fn get_u128_ne(&mut self) -> u128;
    fn get_i128(&mut self) -> i128;
    fn get_i128_le(&mut self) -> i128;
    fn get_i128_ne(&mut self) -> i128;
    fn get_uint(&mut self, nbytes: usize) -> u64;
    fn get_uint_le(&mut self, nbytes: usize) -> u64;
    fn get_uint_ne(&mut self, nbytes: usize) -> u64;
    fn get_int(&mut self, nbytes: usize) -> i64;
    fn get_int_le(&mut self, nbytes: usize) -> i64;
    fn get_int_ne(&mut self, nbytes: usize) -> i64;
    fn get_f32(&mut self) -> f32;
    fn get_f32_le(&mut self) -> f32;
    fn get_f32_ne(&mut self) -> f32;
    fn get_f64(&mut self) -> f64;
    fn get_f64_le(&mut self) -> f64;
    fn get_f64_ne(&mut self) -> f64;
    fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError>;
    fn try_get_u8(&mut self) -> Result<u8, TryGetError>;
    fn try_get_i8(&mut self) -> Result<i8, TryGetError>;
    fn try_get_u16(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_le(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError>;
    fn try_get_i16(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_le(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError>;
    fn try_get_u32(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_le(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError>;
    fn try_get_i32(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_le(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError>;
    fn try_get_u64(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_le(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError>;
    fn try_get_i64(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_le(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError>;
    fn try_get_u128(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_le(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError>;
    fn try_get_i128(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_le(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError>;
    fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_f32(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_le(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f64(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_le(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError>;
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes;
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
    fn chain<U: Buf>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn reader(self) -> Reader<Self>
    where
        Self: Sized,
    {
        reader::new(self)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}

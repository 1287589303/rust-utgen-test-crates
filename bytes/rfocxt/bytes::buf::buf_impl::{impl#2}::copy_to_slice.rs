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
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        if dst.is_empty() {
            return 0;
        }
        if self.has_remaining() {
            dst[0] = IoSlice::new(self.chunk());
            1
        } else {
            0
        }
    }
    fn advance(&mut self, cnt: usize);
    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        self.try_copy_to_slice(dst).unwrap_or_else(|error| panic_advance(&error));
    }
    fn get_u8(&mut self) -> u8 {
        if self.remaining() < 1 {
            panic_advance(
                &TryGetError {
                    requested: 1,
                    available: 0,
                },
            )
        }
        let ret = self.chunk()[0];
        self.advance(1);
        ret
    }
    fn get_i8(&mut self) -> i8 {
        if self.remaining() < 1 {
            panic_advance(
                &TryGetError {
                    requested: 1,
                    available: 0,
                },
            );
        }
        let ret = self.chunk()[0] as i8;
        self.advance(1);
        ret
    }
    fn get_u16(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_be_bytes);
    }
    fn get_u16_le(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_le_bytes);
    }
    fn get_u16_ne(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_ne_bytes);
    }
    fn get_i16(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_be_bytes);
    }
    fn get_i16_le(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_le_bytes);
    }
    fn get_i16_ne(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_ne_bytes);
    }
    fn get_u32(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_be_bytes);
    }
    fn get_u32_le(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_le_bytes);
    }
    fn get_u32_ne(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_ne_bytes);
    }
    fn get_i32(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_be_bytes);
    }
    fn get_i32_le(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_le_bytes);
    }
    fn get_i32_ne(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_ne_bytes);
    }
    fn get_u64(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_be_bytes);
    }
    fn get_u64_le(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_le_bytes);
    }
    fn get_u64_ne(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_ne_bytes);
    }
    fn get_i64(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_be_bytes);
    }
    fn get_i64_le(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_le_bytes);
    }
    fn get_i64_ne(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_ne_bytes);
    }
    fn get_u128(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_be_bytes);
    }
    fn get_u128_le(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_le_bytes);
    }
    fn get_u128_ne(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_ne_bytes);
    }
    fn get_i128(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_be_bytes);
    }
    fn get_i128_le(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_le_bytes);
    }
    fn get_i128_ne(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_ne_bytes);
    }
    fn get_uint(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(be => self, u64, nbytes);
    }
    fn get_uint_le(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(le => self, u64, nbytes);
    }
    fn get_uint_ne(&mut self, nbytes: usize) -> u64 {
        if cfg!(target_endian = "big") {
            self.get_uint(nbytes)
        } else {
            self.get_uint_le(nbytes)
        }
    }
    fn get_int(&mut self, nbytes: usize) -> i64 {
        sign_extend(self.get_uint(nbytes), nbytes)
    }
    fn get_int_le(&mut self, nbytes: usize) -> i64 {
        sign_extend(self.get_uint_le(nbytes), nbytes)
    }
    fn get_int_ne(&mut self, nbytes: usize) -> i64 {
        if cfg!(target_endian = "big") {
            self.get_int(nbytes)
        } else {
            self.get_int_le(nbytes)
        }
    }
    fn get_f32(&mut self) -> f32 {
        f32::from_bits(self.get_u32())
    }
    fn get_f32_le(&mut self) -> f32 {
        f32::from_bits(self.get_u32_le())
    }
    fn get_f32_ne(&mut self) -> f32 {
        f32::from_bits(self.get_u32_ne())
    }
    fn get_f64(&mut self) -> f64 {
        f64::from_bits(self.get_u64())
    }
    fn get_f64_le(&mut self) -> f64 {
        f64::from_bits(self.get_u64_le())
    }
    fn get_f64_ne(&mut self) -> f64 {
        f64::from_bits(self.get_u64_ne())
    }
    fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError> {
        if self.remaining() < dst.len() {
            return Err(TryGetError {
                requested: dst.len(),
                available: self.remaining(),
            });
        }
        while !dst.is_empty() {
            let src = self.chunk();
            let cnt = usize::min(src.len(), dst.len());
            dst[..cnt].copy_from_slice(&src[..cnt]);
            dst = &mut dst[cnt..];
            self.advance(cnt);
        }
        Ok(())
    }
    fn try_get_u8(&mut self) -> Result<u8, TryGetError> {
        if self.remaining() < 1 {
            return Err(TryGetError {
                requested: 1,
                available: self.remaining(),
            });
        }
        let ret = self.chunk()[0];
        self.advance(1);
        Ok(ret)
    }
    fn try_get_i8(&mut self) -> Result<i8, TryGetError> {
        if self.remaining() < 1 {
            return Err(TryGetError {
                requested: 1,
                available: self.remaining(),
            });
        }
        let ret = self.chunk()[0] as i8;
        self.advance(1);
        Ok(ret)
    }
    fn try_get_u16(&mut self) -> Result<u16, TryGetError> {
        buf_try_get_impl!(self, u16::from_be_bytes)
    }
    fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> {
        buf_try_get_impl!(self, u16::from_le_bytes)
    }
    fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> {
        buf_try_get_impl!(self, u16::from_ne_bytes)
    }
    fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
        buf_try_get_impl!(self, i16::from_be_bytes)
    }
    fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> {
        buf_try_get_impl!(self, i16::from_le_bytes)
    }
    fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> {
        buf_try_get_impl!(self, i16::from_ne_bytes)
    }
    fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
        buf_try_get_impl!(self, u32::from_be_bytes)
    }
    fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> {
        buf_try_get_impl!(self, u32::from_le_bytes)
    }
    fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> {
        buf_try_get_impl!(self, u32::from_ne_bytes)
    }
    fn try_get_i32(&mut self) -> Result<i32, TryGetError> {
        buf_try_get_impl!(self, i32::from_be_bytes)
    }
    fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> {
        buf_try_get_impl!(self, i32::from_le_bytes)
    }
    fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> {
        buf_try_get_impl!(self, i32::from_ne_bytes)
    }
    fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
        buf_try_get_impl!(self, u64::from_be_bytes)
    }
    fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
        buf_try_get_impl!(self, u64::from_le_bytes)
    }
    fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> {
        buf_try_get_impl!(self, u64::from_ne_bytes)
    }
    fn try_get_i64(&mut self) -> Result<i64, TryGetError> {
        buf_try_get_impl!(self, i64::from_be_bytes)
    }
    fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> {
        buf_try_get_impl!(self, i64::from_le_bytes)
    }
    fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> {
        buf_try_get_impl!(self, i64::from_ne_bytes)
    }
    fn try_get_u128(&mut self) -> Result<u128, TryGetError> {
        buf_try_get_impl!(self, u128::from_be_bytes)
    }
    fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> {
        buf_try_get_impl!(self, u128::from_le_bytes)
    }
    fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> {
        buf_try_get_impl!(self, u128::from_ne_bytes)
    }
    fn try_get_i128(&mut self) -> Result<i128, TryGetError> {
        buf_try_get_impl!(self, i128::from_be_bytes)
    }
    fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> {
        buf_try_get_impl!(self, i128::from_le_bytes)
    }
    fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> {
        buf_try_get_impl!(self, i128::from_ne_bytes)
    }
    fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        buf_try_get_impl!(be => self, u64, nbytes);
    }
    fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        buf_try_get_impl!(le => self, u64, nbytes);
    }
    fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        if cfg!(target_endian = "big") {
            self.try_get_uint(nbytes)
        } else {
            self.try_get_uint_le(nbytes)
        }
    }
    fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        buf_try_get_impl!(be => self, i64, nbytes);
    }
    fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        buf_try_get_impl!(le => self, i64, nbytes);
    }
    fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        if cfg!(target_endian = "big") {
            self.try_get_int(nbytes)
        } else {
            self.try_get_int_le(nbytes)
        }
    }
    fn try_get_f32(&mut self) -> Result<f32, TryGetError> {
        Ok(f32::from_bits(self.try_get_u32()?))
    }
    fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> {
        Ok(f32::from_bits(self.try_get_u32_le()?))
    }
    fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> {
        Ok(f32::from_bits(self.try_get_u32_ne()?))
    }
    fn try_get_f64(&mut self) -> Result<f64, TryGetError> {
        Ok(f64::from_bits(self.try_get_u64()?))
    }
    fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> {
        Ok(f64::from_bits(self.try_get_u64_le()?))
    }
    fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> {
        Ok(f64::from_bits(self.try_get_u64_ne()?))
    }
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        use super::BufMut;
        if self.remaining() < len {
            panic_advance(
                &TryGetError {
                    requested: len,
                    available: self.remaining(),
                },
            );
        }
        let mut ret = crate::BytesMut::with_capacity(len);
        ret.put(self.take(len));
        ret.freeze()
    }
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
pub unsafe trait BufMut {
    fn remaining_mut(&self) -> usize;
    unsafe fn advance_mut(&mut self, cnt: usize);
    #[inline]
    fn has_remaining_mut(&self) -> bool;
    #[cfg_attr(docsrs, doc(alias = "bytes_mut"))]
    fn chunk_mut(&mut self) -> &mut UninitSlice;
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized;
    #[inline]
    fn put_slice(&mut self, mut src: &[u8]);
    #[inline]
    fn put_bytes(&mut self, val: u8, mut cnt: usize);
    #[inline]
    fn put_u8(&mut self, n: u8);
    #[inline]
    fn put_i8(&mut self, n: i8);
    #[inline]
    fn put_u16(&mut self, n: u16);
    #[inline]
    fn put_u16_le(&mut self, n: u16);
    #[inline]
    fn put_u16_ne(&mut self, n: u16);
    #[inline]
    fn put_i16(&mut self, n: i16);
    #[inline]
    fn put_i16_le(&mut self, n: i16);
    #[inline]
    fn put_i16_ne(&mut self, n: i16);
    #[inline]
    fn put_u32(&mut self, n: u32);
    #[inline]
    fn put_u32_le(&mut self, n: u32);
    #[inline]
    fn put_u32_ne(&mut self, n: u32);
    #[inline]
    fn put_i32(&mut self, n: i32);
    #[inline]
    fn put_i32_le(&mut self, n: i32);
    #[inline]
    fn put_i32_ne(&mut self, n: i32);
    #[inline]
    fn put_u64(&mut self, n: u64);
    #[inline]
    fn put_u64_le(&mut self, n: u64);
    #[inline]
    fn put_u64_ne(&mut self, n: u64);
    #[inline]
    fn put_i64(&mut self, n: i64);
    #[inline]
    fn put_i64_le(&mut self, n: i64);
    #[inline]
    fn put_i64_ne(&mut self, n: i64);
    #[inline]
    fn put_u128(&mut self, n: u128);
    #[inline]
    fn put_u128_le(&mut self, n: u128);
    #[inline]
    fn put_u128_ne(&mut self, n: u128);
    #[inline]
    fn put_i128(&mut self, n: i128);
    #[inline]
    fn put_i128_le(&mut self, n: i128);
    #[inline]
    fn put_i128_ne(&mut self, n: i128);
    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_ne(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_ne(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_f32(&mut self, n: f32);
    #[inline]
    fn put_f32_le(&mut self, n: f32);
    #[inline]
    fn put_f32_ne(&mut self, n: f32);
    #[inline]
    fn put_f64(&mut self, n: f64);
    #[inline]
    fn put_f64_le(&mut self, n: f64);
    #[inline]
    fn put_f64_ne(&mut self, n: f64);
    #[inline]
    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Sized,
    {
        limit::new(self, limit)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[inline]
    fn writer(self) -> Writer<Self>
    where
        Self: Sized,
    {
        writer::new(self)
    }
    #[inline]
    fn chain_mut<U: BufMut>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}
impl Buf for &[u8] {
    #[inline]
    fn remaining(&self) -> usize {}
    #[inline]
    fn chunk(&self) -> &[u8] {}
    #[inline]
    fn advance(&mut self, cnt: usize) {}
    #[inline]
    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        if self.len() < dst.len() {
            panic_advance(
                &TryGetError {
                    requested: dst.len(),
                    available: self.len(),
                },
            );
        }
        dst.copy_from_slice(&self[..dst.len()]);
        self.advance(dst.len());
    }
}
#[cold]
fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}", error_info.available,
        error_info.requested
    );
}

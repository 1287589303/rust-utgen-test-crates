use crate::guts::ChaCha;
use core::fmt;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
const BUF_BLOCKS: u8 = 4;
const BLOCK_WORDS: u8 = 16;
#[repr(transparent)]
pub struct Array64<T>([T; 64]);
impl<T> Clone for Array64<T>
where
    T: Copy + Default,
{
    fn clone(&self) -> Self {
        let mut new = Self::default();
        new.0.copy_from_slice(&self.0);
        new
    }
}

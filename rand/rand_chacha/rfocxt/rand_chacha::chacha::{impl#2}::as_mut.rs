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
impl<T> AsMut<[T]> for Array64<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

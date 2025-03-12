use ppv_lite86::{dispatch, dispatch_light128};
pub use ppv_lite86::Machine;
use ppv_lite86::{
    vec128_storage, ArithOps, BitOps32, LaneWords4, MultiLane, StoreBytes, Vec4, Vec4Ext,
    Vector,
};
pub(crate) const BLOCK: usize = 16;
pub(crate) const BLOCK64: u64 = BLOCK as u64;
const LOG2_BUFBLOCKS: u64 = 2;
const BUFBLOCKS: u64 = 1 << LOG2_BUFBLOCKS;
pub(crate) const BUFSZ64: u64 = BLOCK64 * BUFBLOCKS;
pub(crate) const BUFSZ: usize = BUFSZ64 as usize;
const STREAM_PARAM_NONCE: u32 = 1;
const STREAM_PARAM_BLOCK: u32 = 0;
#[derive(Clone, PartialEq, Eq)]
pub struct ChaCha {
    pub(crate) b: vec128_storage,
    pub(crate) c: vec128_storage,
    pub(crate) d: vec128_storage,
}
impl ChaCha {
    #[inline(always)]
    pub fn new(key: &[u8; 32], nonce: &[u8]) -> Self {
        init_chacha(key, nonce)
    }
    #[inline(always)]
    pub fn refill4(&mut self, drounds: u32, out: &mut [u32; BUFSZ]) {}
    #[inline(always)]
    pub fn set_block_pos(&mut self, value: u64) {
        set_stream_param(self, STREAM_PARAM_BLOCK, value)
    }
    #[inline(always)]
    pub fn get_block_pos(&self) -> u64 {}
    #[inline(always)]
    pub fn set_nonce(&mut self, value: u64) {}
    #[inline(always)]
    pub fn get_nonce(&self) -> u64 {}
    #[inline(always)]
    pub fn get_seed(&self) -> [u8; 32] {}
}

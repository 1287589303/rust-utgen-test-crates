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
#[derive(Clone)]
pub struct State<V> {
    pub(crate) a: V,
    pub(crate) b: V,
    pub(crate) c: V,
    pub(crate) d: V,
}
#[inline(always)]
pub(crate) fn diagonalize<V: LaneWords4>(mut x: State<V>) -> State<V> {
    x.b = x.b.shuffle_lane_words3012();
    x.c = x.c.shuffle_lane_words2301();
    x.d = x.d.shuffle_lane_words1230();
    x
}

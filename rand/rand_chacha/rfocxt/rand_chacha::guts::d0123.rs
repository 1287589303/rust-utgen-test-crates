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
#[inline(always)]
#[cfg(target_endian = "little")]
fn d0123<Mach: Machine>(m: Mach, d: vec128_storage) -> Mach::u32x4x4 {
    let d0: Mach::u64x2 = m.unpack(d);
    let incr = Mach::u64x2x4::from_lanes([
        m.vec([0, 0]),
        m.vec([1, 0]),
        m.vec([2, 0]),
        m.vec([3, 0]),
    ]);
    m.unpack((Mach::u64x2x4::from_lanes([d0, d0, d0, d0]) + incr).into())
}

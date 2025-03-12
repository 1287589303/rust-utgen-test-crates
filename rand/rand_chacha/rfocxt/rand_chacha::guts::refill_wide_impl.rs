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
#[allow(clippy::many_single_char_names)]
#[inline(always)]
fn refill_wide_impl<Mach: Machine>(
    m: Mach,
    state: &mut ChaCha,
    drounds: u32,
    out: &mut [u32; BUFSZ],
) {
    let k = m.vec([0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574]);
    let b = m.unpack(state.b);
    let c = m.unpack(state.c);
    let mut x = State {
        a: Mach::u32x4x4::from_lanes([k, k, k, k]),
        b: Mach::u32x4x4::from_lanes([b, b, b, b]),
        c: Mach::u32x4x4::from_lanes([c, c, c, c]),
        d: d0123(m, state.d),
    };
    for _ in 0..drounds {
        x = round(x);
        x = undiagonalize(round(diagonalize(x)));
    }
    let kk = Mach::u32x4x4::from_lanes([k, k, k, k]);
    let sb = m.unpack(state.b);
    let sb = Mach::u32x4x4::from_lanes([sb, sb, sb, sb]);
    let sc = m.unpack(state.c);
    let sc = Mach::u32x4x4::from_lanes([sc, sc, sc, sc]);
    let sd = d0123(m, state.d);
    let results = Mach::u32x4x4::transpose4(x.a + kk, x.b + sb, x.c + sc, x.d + sd);
    out[0..16].copy_from_slice(&results.0.to_scalars());
    out[16..32].copy_from_slice(&results.1.to_scalars());
    out[32..48].copy_from_slice(&results.2.to_scalars());
    out[48..64].copy_from_slice(&results.3.to_scalars());
    state.d = add_pos(m, sd.to_lanes()[0], 4).into();
}
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
#[inline(always)]
#[cfg(target_endian = "little")]
fn add_pos<Mach: Machine>(m: Mach, d: Mach::u32x4, i: u64) -> Mach::u32x4 {
    let d0: Mach::u64x2 = m.unpack(d.into());
    let incr = m.vec([i, 0]);
    m.unpack((d0 + incr).into())
}

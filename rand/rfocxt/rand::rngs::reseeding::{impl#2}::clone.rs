use core::mem::size_of_val;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[derive(Debug)]
pub struct ReseedingRng<R, Rsdr>(
    BlockRng<ReseedingCore<R, Rsdr>>,
)
where
    R: BlockRngCore + SeedableRng,
    Rsdr: TryRngCore;
#[derive(Debug)]
struct ReseedingCore<R, Rsdr> {
    inner: R,
    reseeder: Rsdr,
    threshold: i64,
    bytes_until_reseed: i64,
}
impl<R, Rsdr> Clone for ReseedingRng<R, Rsdr>
where
    R: BlockRngCore + SeedableRng + Clone,
    Rsdr: TryRngCore + Clone,
{
    fn clone(&self) -> ReseedingRng<R, Rsdr> {
        ReseedingRng(BlockRng::new(self.0.core.clone()))
    }
}

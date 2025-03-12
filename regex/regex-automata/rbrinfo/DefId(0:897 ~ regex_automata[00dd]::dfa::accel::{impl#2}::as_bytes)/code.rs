pub fn as_bytes(&self) -> &[u8] {
        let accels = self.accels.as_ref();
        // SAFETY: This is safe because accels is a just a slice of AccelTy,
        // and u8 always has a smaller alignment.
        unsafe {
            core::slice::from_raw_parts(
                accels.as_ptr().cast::<u8>(),
                accels.len() * ACCEL_TY_SIZE,
            )
        }
    }
pub fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Accels<&'a [AccelTy]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();

        let (accel_len, _) =
            wire::try_read_u32_as_usize(slice, "accelerators length")?;
        // The accelerator length is part of the accel_tys slice that
        // we deserialize. This is perhaps a bit idiosyncratic. It would
        // probably be better to split out the length into a real field.

        let accel_tys_len = wire::add(
            wire::mul(accel_len, 2, "total number of accelerator accel_tys")?,
            1,
            "total number of accel_tys",
        )?;
        let accel_tys_bytes_len = wire::mul(
            ACCEL_TY_SIZE,
            accel_tys_len,
            "total number of bytes in accelerators",
        )?;
        wire::check_slice_len(slice, accel_tys_bytes_len, "accelerators")?;
        wire::check_alignment::<AccelTy>(slice)?;
        let accel_tys = &slice[..accel_tys_bytes_len];
        slice = &slice[accel_tys_bytes_len..];
        // SAFETY: We've checked the length and alignment above, and since
        // slice is just bytes and AccelTy is just a u32, we can safely cast to
        // a slice of &[AccelTy].
        let accels = unsafe {
            core::slice::from_raw_parts(
                accel_tys.as_ptr().cast::<AccelTy>(),
                accel_tys_len,
            )
        };
        Ok((Accels { accels }, slice.as_ptr().as_usize() - slice_start))
    }
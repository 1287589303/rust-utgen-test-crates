fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let mut nw = 0;
        nw += wire::write_label(LABEL, &mut dst[nw..])?;
        nw += wire::write_endianness_check::<E>(&mut dst[nw..])?;
        nw += wire::write_version::<E>(VERSION, &mut dst[nw..])?;
        nw += {
            // Currently unused, intended for future flexibility
            E::write_u32(0, &mut dst[nw..]);
            size_of::<u32>()
        };
        nw += self.flags.write_to::<E>(&mut dst[nw..])?;
        nw += self.tt.write_to::<E>(&mut dst[nw..])?;
        nw += self.st.write_to::<E>(&mut dst[nw..])?;
        nw += self.special.write_to::<E>(&mut dst[nw..])?;
        nw += self.quitset.write_to::<E>(&mut dst[nw..])?;
        Ok(nw)
    }
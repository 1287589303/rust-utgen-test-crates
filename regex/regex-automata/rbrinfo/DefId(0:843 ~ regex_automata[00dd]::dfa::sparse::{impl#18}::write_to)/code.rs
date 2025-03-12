fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small(
                "sparse state transitions",
            ));
        }

        let ntrans =
            if self.is_match { self.ntrans | (1 << 15) } else { self.ntrans };
        E::write_u16(u16::try_from(ntrans).unwrap(), dst);
        dst = &mut dst[size_of::<u16>()..];

        dst[..self.input_ranges.len()].copy_from_slice(self.input_ranges);
        dst = &mut dst[self.input_ranges.len()..];

        for i in 0..self.ntrans {
            E::write_u32(self.next_at(i).as_u32(), dst);
            dst = &mut dst[StateID::SIZE..];
        }

        if self.is_match {
            E::write_u32(u32::try_from(self.pattern_len()).unwrap(), dst);
            dst = &mut dst[size_of::<u32>()..];
            for i in 0..self.pattern_len() {
                let pid = self.pattern_id(i);
                E::write_u32(pid.as_u32(), dst);
                dst = &mut dst[PatternID::SIZE..];
            }
        }

        dst[0] = u8::try_from(self.accel.len()).unwrap();
        dst[1..][..self.accel.len()].copy_from_slice(self.accel);

        Ok(nwrite)
    }
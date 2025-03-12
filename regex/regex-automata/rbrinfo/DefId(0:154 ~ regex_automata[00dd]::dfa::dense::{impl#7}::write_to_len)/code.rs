pub fn write_to_len(&self) -> usize {
        wire::write_label_len(LABEL)
        + wire::write_endianness_check_len()
        + wire::write_version_len()
        + size_of::<u32>() // unused, intended for future flexibility
        + self.flags.write_to_len()
        + self.tt.write_to_len()
        + self.st.write_to_len()
        + self.ms.write_to_len()
        + self.special.write_to_len()
        + self.accels.write_to_len()
        + self.quitset.write_to_len()
    }
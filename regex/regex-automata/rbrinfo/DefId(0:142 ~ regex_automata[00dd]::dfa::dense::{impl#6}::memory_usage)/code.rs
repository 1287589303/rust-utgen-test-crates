pub fn memory_usage(&self) -> usize {
        self.tt.memory_usage()
            + self.st.memory_usage()
            + self.ms.memory_usage()
            + self.accels.memory_usage()
    }
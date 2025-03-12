fn ctrl_slice(&mut self) -> &mut [Tag] {
        // SAFETY: We've intiailized all control bytes, and have the correct number.
        unsafe { slice::from_raw_parts_mut(self.ctrl.as_ptr().cast(), self.num_ctrl_bytes()) }
    }
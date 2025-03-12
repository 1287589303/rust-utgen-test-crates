fn take_fragment(&mut self) -> Option<String> {
        self.fragment_start.take().map(|start| {
            debug_assert!(self.byte_at(start) == b'#');
            let fragment = self.slice(start + 1..).to_owned();
            self.serialization.truncate(start as usize);
            fragment
        })
    }
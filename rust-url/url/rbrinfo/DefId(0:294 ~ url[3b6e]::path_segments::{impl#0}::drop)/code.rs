fn drop(&mut self) {
        self.url
            .restore_after_path(self.old_after_path_position, &self.after_path)
    }
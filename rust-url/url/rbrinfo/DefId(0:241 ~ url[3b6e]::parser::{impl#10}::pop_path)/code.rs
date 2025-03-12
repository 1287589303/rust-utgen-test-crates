fn pop_path(&mut self, scheme_type: SchemeType, path_start: usize) {
        if self.serialization.len() > path_start {
            let slash_position = self.serialization[path_start..].rfind('/').unwrap();
            // + 1 since rfind returns the position before the slash.
            let segment_start = path_start + slash_position + 1;
            // Don’t pop a Windows drive letter
            if !(scheme_type.is_file()
                && is_normalized_windows_drive_letter(&self.serialization[segment_start..]))
            {
                self.serialization.truncate(segment_start);
            }
        }
    }
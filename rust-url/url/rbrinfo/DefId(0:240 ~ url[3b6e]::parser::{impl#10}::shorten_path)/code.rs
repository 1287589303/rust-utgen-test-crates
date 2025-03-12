fn shorten_path(&mut self, scheme_type: SchemeType, path_start: usize) {
        // If path is empty, then return.
        if self.serialization.len() == path_start {
            return;
        }
        // If url’s scheme is "file", path’s size is 1, and path[0] is a normalized Windows drive letter, then return.
        if scheme_type.is_file()
            && is_normalized_windows_drive_letter(&self.serialization[path_start..])
        {
            return;
        }
        // Remove path’s last item.
        self.pop_path(scheme_type, path_start);
    }
fn last_slash_can_be_removed(serialization: &str, path_start: usize) -> bool {
        let url_before_segment = &serialization[..serialization.len() - 1];
        if let Some(segment_before_start) = url_before_segment.rfind('/') {
            // Do not remove the root slash
            segment_before_start >= path_start
                // Or a windows drive letter slash
                && !path_starts_with_windows_drive_letter(&serialization[segment_before_start..])
        } else {
            false
        }
    }
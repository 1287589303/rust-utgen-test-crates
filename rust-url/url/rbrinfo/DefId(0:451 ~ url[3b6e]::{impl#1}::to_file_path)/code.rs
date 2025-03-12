pub fn to_file_path(&self) -> Result<PathBuf, ()> {
        if let Some(segments) = self.path_segments() {
            let host = match self.host() {
                None | Some(Host::Domain("localhost")) => None,
                Some(_) if cfg!(windows) && self.scheme() == "file" => {
                    Some(&self.serialization[self.host_start as usize..self.host_end as usize])
                }
                _ => return Err(()),
            };

            let str_len = self.as_str().len();
            let estimated_capacity = if cfg!(target_os = "redox") {
                let scheme_len = self.scheme().len();
                let file_scheme_len = "file".len();
                // remove only // because it still has file:
                if scheme_len < file_scheme_len {
                    let scheme_diff = file_scheme_len - scheme_len;
                    (str_len + scheme_diff).saturating_sub(2)
                } else {
                    let scheme_diff = scheme_len - file_scheme_len;
                    str_len.saturating_sub(scheme_diff + 2)
                }
            } else if cfg!(windows) {
                // remove scheme: - has posssible \\ for hostname
                str_len.saturating_sub(self.scheme().len() + 1)
            } else {
                // remove scheme://
                str_len.saturating_sub(self.scheme().len() + 3)
            };
            return file_url_segments_to_pathbuf(estimated_capacity, host, segments);
        }
        Err(())
    }
pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> {
        let mut serialization = "file://".to_owned();
        let host_start = serialization.len() as u32;
        let (host_end, host) = path_to_file_url_segments(path.as_ref(), &mut serialization)?;
        Ok(Url {
            serialization,
            scheme_end: "file".len() as u32,
            username_end: host_start,
            host_start,
            host_end,
            host,
            port: None,
            path_start: host_end,
            query_start: None,
            fragment_start: None,
        })
    }
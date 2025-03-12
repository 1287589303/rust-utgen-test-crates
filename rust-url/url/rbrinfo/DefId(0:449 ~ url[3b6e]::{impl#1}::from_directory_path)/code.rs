pub fn from_directory_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> {
        let mut url = Url::from_file_path(path)?;
        if !url.serialization.ends_with('/') {
            url.serialization.push('/')
        }
        Ok(url)
    }
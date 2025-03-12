pub fn make_relative(&self, url: &Url) -> Option<String> {
        if self.cannot_be_a_base() {
            return None;
        }

        // Scheme, host and port need to be the same
        if self.scheme() != url.scheme() || self.host() != url.host() || self.port() != url.port() {
            return None;
        }

        // We ignore username/password at this point

        // The path has to be transformed
        let mut relative = String::new();

        // Extract the filename of both URIs, these need to be handled separately
        fn extract_path_filename(s: &str) -> (&str, &str) {
            let last_slash_idx = s.rfind('/').unwrap_or(0);
            let (path, filename) = s.split_at(last_slash_idx);
            if filename.is_empty() {
                (path, "")
            } else {
                (path, &filename[1..])
            }
        }

        let (base_path, base_filename) = extract_path_filename(self.path());
        let (url_path, url_filename) = extract_path_filename(url.path());

        let mut base_path = base_path.split('/').peekable();
        let mut url_path = url_path.split('/').peekable();

        // Skip over the common prefix
        while base_path.peek().is_some() && base_path.peek() == url_path.peek() {
            base_path.next();
            url_path.next();
        }

        // Add `..` segments for the remainder of the base path
        for base_path_segment in base_path {
            // Skip empty last segments
            if base_path_segment.is_empty() {
                break;
            }

            if !relative.is_empty() {
                relative.push('/');
            }

            relative.push_str("..");
        }

        // Append the remainder of the other URI
        for url_path_segment in url_path {
            if !relative.is_empty() {
                relative.push('/');
            }

            relative.push_str(url_path_segment);
        }

        // Add the filename if they are not the same
        if !relative.is_empty() || base_filename != url_filename {
            // If the URIs filename is empty this means that it was a directory
            // so we'll have to append a '/'.
            //
            // Otherwise append it directly as the new filename.
            if url_filename.is_empty() {
                relative.push('/');
            } else {
                if !relative.is_empty() {
                    relative.push('/');
                }
                relative.push_str(url_filename);
            }
        }

        // Query and fragment are only taken from the other URI
        if let Some(query) = url.query() {
            relative.push('?');
            relative.push_str(query);
        }

        if let Some(fragment) = url.fragment() {
            relative.push('#');
            relative.push_str(fragment);
        }

        Some(relative)
    }
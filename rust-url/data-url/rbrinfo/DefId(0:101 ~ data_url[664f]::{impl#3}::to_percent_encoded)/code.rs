pub fn to_percent_encoded(&self) -> String {
        let mut string = String::new();
        for byte in self.0.bytes() {
            match byte {
                // Ignore ASCII tabs or newlines like the URL parser would
                b'\t' | b'\n' | b'\r' => continue,
                // https://url.spec.whatwg.org/#fragment-percent-encode-set
                b'\0'..=b' ' | b'"' | b'<' | b'>' | b'`' | b'\x7F'..=b'\xFF' => {
                    percent_encode(byte, &mut string)
                }
                // Printable ASCII
                _ => string.push(byte as char),
            }
        }
        string
    }
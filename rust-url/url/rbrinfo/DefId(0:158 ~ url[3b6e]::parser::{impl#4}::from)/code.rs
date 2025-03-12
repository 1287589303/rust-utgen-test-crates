fn from(s: T) -> Self {
        match s.as_ref() {
            "http" | "https" | "ws" | "wss" | "ftp" => SchemeType::SpecialNotFile,
            "file" => SchemeType::File,
            _ => SchemeType::NotSpecial,
        }
    }
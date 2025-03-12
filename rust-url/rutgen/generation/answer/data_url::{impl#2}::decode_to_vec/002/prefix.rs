// Answer 0

#[test]
fn test_decode_to_vec_valid_base64_text_plain() {
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: String::from("text"),
            subtype: String::from("plain"),
            parameters: vec![],
        },
        base64: true,
        encoded_body_plus_fragment: "aGVsbG8sIHdvcmxkIQ==#fragment",
    };
    let _ = data_url.decode_to_vec();
}

#[test]
fn test_decode_to_vec_valid_base64_image_png() {
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: String::from("image"),
            subtype: String::from("png"),
            parameters: vec![],
        },
        base64: true,
        encoded_body_plus_fragment: "iVBORw0KGgoAAAANSUhEUgAAAAUA\nAAAAF...",
    };
    let _ = data_url.decode_to_vec();
}

#[test]
fn test_decode_to_vec_without_base64() {
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: String::from("text"),
            subtype: String::from("html"),
            parameters: vec![],
        },
        base64: false,
        encoded_body_plus_fragment: "Hello, World!#fragment",
    };
    let _ = data_url.decode_to_vec();
}

#[test]
fn test_decode_to_vec_valid_base64_with_no_fragment() {
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: String::from("application"),
            subtype: String::from("json"),
            parameters: vec![],
        },
        base64: true,
        encoded_body_plus_fragment: "eyJ0ZXN0IjoiZm9vIn0K", // Base64 for {"test":"foo"}
    };
    let _ = data_url.decode_to_vec();
}


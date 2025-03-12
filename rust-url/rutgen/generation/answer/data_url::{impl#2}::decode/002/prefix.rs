// Answer 0

#[test]
fn test_decode_without_base64_valid_input() {
    let encoded_body = "some%20encoded%20string";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![("charset".to_string(), "utf-8".to_string())],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        // Here we can just ignore the written bytes for the test
        let _ = bytes;
        Ok(())
    });
}

#[test]
fn test_decode_without_base64_with_fragment() {
    let encoded_body = "some%20encoded%20string#fragment";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![("charset".to_string(), "utf-8".to_string())],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        let _ = bytes;
        Ok(())
    });
}

#[test]
fn test_decode_without_base64_empty_encoded_body() {
    let encoded_body = "";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        let _ = bytes;
        Ok(())
    });
}

#[test]
fn test_decode_without_base64_handle_percent_character() {
    let encoded_body = "some%encoded%20string";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        let _ = bytes;
        Ok(())
    });
}

#[test]
fn test_decode_without_base64_handle_fragment_only() {
    let encoded_body = "#fragmentonly";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        let _ = bytes;
        Ok(())
    });
}

#[test]
fn test_decode_without_base64_invalid_url() {
    let encoded_body = "invalid%url%#";
    let data_url = DataUrl {
        mime_type: mime::Mime {
            type_: "text".to_string(),
            subtype: "plain".to_string(),
            parameters: vec![],
        },
        base64: false,
        encoded_body_plus_fragment: encoded_body,
    };

    let _result = data_url.decode(|bytes| {
        let _ = bytes;
        Ok(())
    });
}

